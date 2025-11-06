package main

import (
	"bytes"
	"context"
	"embed"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
	"sync"

	"github.com/wailsapp/wails/v2"
	"github.com/wailsapp/wails/v2/pkg/logger"
	"github.com/wailsapp/wails/v2/pkg/options"
	"github.com/wailsapp/wails/v2/pkg/options/assetserver"
)

//go:embed frontend/dist
var assets embed.FS

type ProviderConfig struct {
	Type     string `json:"type"`
	Name     string `json:"name"`
	APIKey   string `json:"apiKey"`
	Endpoint string `json:"endpoint"`
	Model    string `json:"model"`
}

type Provider interface {
	SendRequest(prompt string, temperature float64, maxTokens int) (string, error)
	GetName() string
}

type OllamaProvider struct {
	config ProviderConfig
	client *http.Client
}

func NewOllamaProvider(config ProviderConfig) *OllamaProvider {
	return &OllamaProvider{
		config: config,
		client: &http.Client{},
	}
}

func (p *OllamaProvider) GetName() string {
	if p.config.Name != "" {
		return p.config.Name
	}
	return "Ollama"
}

func (p *OllamaProvider) SendRequest(prompt string, temperature float64, maxTokens int) (string, error) {
	url := fmt.Sprintf("%s/api/generate", p.config.Endpoint)

	payload := map[string]interface{}{
		"model":  p.config.Model,
		"prompt": prompt,
		"stream": false,
		"options": map[string]interface{}{
			"temperature": temperature,
			"num_predict": maxTokens,
		},
	}

	jsonData, err := json.Marshal(payload)
	if err != nil {
		return "", err
	}

	resp, err := p.client.Post(url, "application/json", bytes.NewBuffer(jsonData))
	if err != nil {
		return "", fmt.Errorf("network error: %v", err)
	}
	defer resp.Body.Close()

	if resp.StatusCode != http.StatusOK {
		body, _ := io.ReadAll(resp.Body)
		return "", fmt.Errorf("HTTP %d: %s", resp.StatusCode, string(body))
	}

	var result map[string]interface{}
	if err := json.NewDecoder(resp.Body).Decode(&result); err != nil {
		return "", fmt.Errorf("invalid response: %v", err)
	}

	response, ok := result["response"].(string)
	if !ok {
		return "", fmt.Errorf("missing 'response' field")
	}

	return response, nil
}

type MockProvider struct {
	config ProviderConfig
}

func NewMockProvider(config ProviderConfig) *MockProvider {
	return &MockProvider{config: config}
}

func (p *MockProvider) GetName() string {
	if p.config.Name != "" {
		return p.config.Name
	}
	return "Mock"
}

func (p *MockProvider) SendRequest(prompt string, temperature float64, maxTokens int) (string, error) {
	return fmt.Sprintf("# Mock AI Response\n\nYou asked: %s\n\n## Code Example\n\n```go\nfunc hello() {\n    fmt.Println(\"Hello from Vibe Coder!\")\n}\n```\n\n## Explanation\n\nThis is a mock response demonstrating the parsing capabilities.", prompt), nil
}

type App struct {
	providers      []Provider
	activeProvider int
	providersMutex sync.RWMutex
}

func NewApp() *App {
	return &App{
		providers:      make([]Provider, 0),
		activeProvider: -1,
	}
}

func (a *App) startup(ctx context.Context) {}

// AddProvider adds a new AI provider
func (a *App) AddProvider(config ProviderConfig) error {
	a.providersMutex.Lock()
	defer a.providersMutex.Unlock()

	var provider Provider
	switch config.Type {
	case "Ollama":
		provider = NewOllamaProvider(config)
	case "Mock":
		provider = NewMockProvider(config)
	default:
		// For now, unsupported providers default to Mock
		provider = NewMockProvider(config)
	}

	a.providers = append(a.providers, provider)

	// Set as active if it's the first provider
	if a.activeProvider == -1 {
		a.activeProvider = 0
	}

	return nil
}

// ListProviders returns names of all configured providers
func (a *App) ListProviders() []string {
	a.providersMutex.RLock()
	defer a.providersMutex.RUnlock()

	names := make([]string, len(a.providers))
	for i, p := range a.providers {
		names[i] = p.GetName()
	}
	return names
}

// SetActiveProvider sets the active provider by index
func (a *App) SetActiveProvider(index int) error {
	a.providersMutex.Lock()
	defer a.providersMutex.Unlock()

	if index < 0 || index >= len(a.providers) {
		return fmt.Errorf("invalid provider index")
	}

	a.activeProvider = index
	return nil
}

// SendPrompt sends a prompt to the active AI provider
func (a *App) SendPrompt(prompt string) (string, error) {
	a.providersMutex.RLock()
	defer a.providersMutex.RUnlock()

	if a.activeProvider == -1 || len(a.providers) == 0 {
		// No provider configured, return mock response
		mock := NewMockProvider(ProviderConfig{Name: "Mock"})
		return mock.SendRequest(prompt, 0.7, 2000)
	}

	provider := a.providers[a.activeProvider]
	return provider.SendRequest(prompt, 0.7, 2000)
}

func main() {
	app := NewApp()

	err := wails.Run(&options.App{
		Title:            "Vibe Coder (Wails)",
		Width:            1200,
		Height:           800,
		LogLevel:         logger.INFO,
		OnStartup:        app.startup,
		Bind:             []interface{}{app},
		AssetServer:      &assetserver.Options{Assets: assets},
		BackgroundColour: &options.RGBA{R: 30, G: 30, B: 30, A: 255},
	})
	if err != nil {
		println("Error:", err.Error())
	}
}

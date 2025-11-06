import React, { useState } from 'react';
import { Editor } from '@monaco-editor/react';
import { FolderOpen, Brain, Cog, PlugZap, Send, X } from 'lucide-react';

// Placeholder for Wails-bound API (after wails generate)
declare global { 
  interface Window { 
    backend?: { 
      App?: { 
        SendPrompt(prompt: string): Promise<string>;
        AddProvider(config: ProviderConfig): Promise<void>;
        ListProviders(): Promise<string[]>;
        SetActiveProvider(index: number): Promise<void>;
      } 
    } 
  } 
}

interface ProviderConfig {
  type: string;
  name: string;
  apiKey: string;
  endpoint: string;
  model: string;
}

const PROVIDER_TYPES = ['Ollama', 'Copilot', 'Gemini', 'Claude', 'Mock'] as const;
type ProviderType = typeof PROVIDER_TYPES[number];

const DEFAULT_ENDPOINTS: Record<ProviderType, string> = {
  Ollama: 'http://localhost:11434',
  Copilot: 'https://api.githubcopilot.com',
  Gemini: 'https://generativelanguage.googleapis.com',
  Claude: 'https://api.anthropic.com',
  Mock: '',
};

const DEFAULT_MODELS: Record<ProviderType, string> = {
  Ollama: 'llama3',
  Copilot: 'gpt-4o-mini',
  Gemini: 'gemini-pro',
  Claude: 'claude-3-opus',
  Mock: 'mock-model-v1',
};

const App: React.FC = () => {
  const [prompt, setPrompt] = useState('');
  const [response, setResponse] = useState<string>('');
  const [loading, setLoading] = useState(false);
  const [style, setStyle] = useState<'vscode' | 'zed'>('vscode');
  const [theme, setTheme] = useState<'dark' | 'light'>('dark');
  const [fontFamily, setFontFamily] = useState<string>('JetBrains Mono');
  const [fontSize, setFontSize] = useState<number>(14);
  const [showProviderDialog, setShowProviderDialog] = useState(false);
  
  // Provider dialog state
  const [providerTypeIndex, setProviderTypeIndex] = useState(0);
  const [providerName, setProviderName] = useState('');
  const [providerApiKey, setProviderApiKey] = useState('');
  const [providerEndpoint, setProviderEndpoint] = useState('');
  const [providerModel, setProviderModel] = useState('');

  const fonts = ['JetBrains Mono', 'Fira Code', 'SF Mono', 'Cascadia Code', 'Menlo'];
  const currentProviderType = PROVIDER_TYPES[providerTypeIndex];

  const cycleFontFamily = () => {
    const idx = fonts.indexOf(fontFamily);
    setFontFamily(fonts[(idx + 1) % fonts.length]);
  };

  const adjustFontSize = (delta: number) => {
    setFontSize(prev => Math.max(10, Math.min(28, prev + delta)));
  };

  const cycleProviderType = () => {
    const nextIndex = (providerTypeIndex + 1) % PROVIDER_TYPES.length;
    setProviderTypeIndex(nextIndex);
    const nextType = PROVIDER_TYPES[nextIndex];
    setProviderEndpoint(DEFAULT_ENDPOINTS[nextType]);
    setProviderModel(DEFAULT_MODELS[nextType]);
  };

  const addProvider = async () => {
    const config: ProviderConfig = {
      type: currentProviderType,
      name: providerName || currentProviderType,
      apiKey: providerApiKey,
      endpoint: providerEndpoint || DEFAULT_ENDPOINTS[currentProviderType],
      model: providerModel || DEFAULT_MODELS[currentProviderType],
    };

    try {
      const api = window.backend?.App;
      if (api?.AddProvider) {
        await api.AddProvider(config);
        console.log('Provider added successfully');
      } else {
        console.log('Adding provider (backend not available):', config);
      }
    } catch (e: any) {
      console.error('Error adding provider:', e);
    }

    setShowProviderDialog(false);
    setProviderName('');
    setProviderApiKey('');
    setProviderEndpoint('');
    setProviderModel('');
  };

  async function send() {
    if (!prompt.trim()) return;
    setLoading(true);
    try {
      const api = window.backend?.App;
      const resp = api ? await api.SendPrompt(prompt) : `Local echo:\n${prompt}`;
      setResponse(resp);
    } catch (e: any) {
      setResponse(`Error: ${e.message || String(e)}`);
    } finally {
      setLoading(false);
    }
  }

  return (
    <div className={`w-screen h-screen flex flex-col ${theme === 'dark' ? 'bg-[#1e1e1e]' : 'bg-white'} text-sm`}>      
      <div className="flex flex-1 overflow-hidden">
        {/* Activity Bar */}
        <div className="w-12 flex flex-col items-center gap-4 py-4 bg-[#2c2c2c] text-gray-300">
          <button><Brain size={20} /></button>
          <button><FolderOpen size={20} /></button>
          <button><PlugZap size={20} /></button>
          <button><Cog size={20} /></button>
        </div>
        {/* Sidebar */}
        <div className="w-56 bg-[#252526] text-gray-300 p-3 flex flex-col gap-2">
          <div className="text-xs uppercase tracking-wide font-semibold">Explorer</div>
          <div className="text-xs opacity-70">Sessions</div>
          <div className="text-xs opacity-70">Providers</div>
          <div className="text-xs opacity-70">Plugins</div>
        </div>
        {/* Main Pane */}
        <div className="flex-1 flex flex-col">
          {/* Tab Bar */}
          <div className="flex items-center h-9 bg-[#2d2d2d] text-gray-200 text-xs">
            <div className="px-3 h-full flex items-center bg-[#1e1e1e] border-r border-[#3c3c3c]">Chat ⨉</div>
          </div>
          {/* Response Area */}
          <div className="flex-1 overflow-auto bg-[#1e1e1e] p-4 space-y-4">
            {response ? (
              <Editor
                theme={theme === 'dark' ? 'vs-dark' : 'light'}
                height="100%"
                defaultLanguage="markdown"
                value={response}
                options={{ readOnly: true, minimap: { enabled: false }, wordWrap: 'on' }}
              />
            ) : (
              <div className="text-gray-500 text-sm">AI responses will appear here...</div>
            )}
          </div>
          {/* Prompt Input */}
          <div className="flex items-center gap-3 p-3 bg-[#252526] border-t border-[#3c3c3c]">
            <textarea
              className="flex-1 bg-[#1e1e1e] rounded-md border border-[#3c3c3c] p-2 text-gray-200 focus:outline-none focus:border-blue-500 resize-none h-20"
              style={{ fontFamily, fontSize: `${fontSize}px` }}
              placeholder="Ask something..."
              value={prompt}
              onChange={(e) => setPrompt(e.target.value)}
            />
            <button
              onClick={send}
              disabled={loading}
              className="flex items-center gap-2 bg-blue-600 hover:bg-blue-500 text-white text-xs font-medium px-4 py-2 rounded-md disabled:opacity-50"
            >
              <Send size={16} /> {loading ? 'Sending...' : 'Send'}
            </button>
          </div>
        </div>
      </div>
      {/* Status Bar */}
      <div className="h-6 bg-[#007acc] flex items-center justify-between px-3 text-xs text-white">
        <div>Vibe Coder Wails · Ready</div>
        <div className="flex items-center gap-3">
          <button onClick={() => setStyle(style === 'vscode' ? 'zed' : 'vscode')} className="opacity-90 hover:opacity-100">{style === 'vscode' ? 'VS Code' : 'Zed'}</button>
          <button onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')} className="opacity-90 hover:opacity-100">{theme === 'dark' ? 'Dark' : 'Light'}</button>
          <button onClick={cycleFontFamily} className="opacity-90 hover:opacity-100">Font</button>
          <button onClick={() => adjustFontSize(-1)} className="opacity-90 hover:opacity-100">A-</button>
          <button onClick={() => adjustFontSize(1)} className="opacity-90 hover:opacity-100">A+</button>
          <button onClick={() => setShowProviderDialog(true)} className="opacity-90 hover:opacity-100">Providers</button>
        </div>
      </div>

      {/* Provider Dialog */}
      {showProviderDialog && (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
          <div className="bg-[#252526] border border-[#3c3c3c] rounded-lg w-[500px] max-h-[90vh] overflow-auto">
            {/* Header */}
            <div className="flex items-center justify-between p-4 border-b border-[#3c3c3c]">
              <h2 className="text-lg font-semibold text-gray-200">Add AI Provider</h2>
              <button onClick={() => setShowProviderDialog(false)} className="text-gray-400 hover:text-gray-200">
                <X size={20} />
              </button>
            </div>
            
            {/* Body */}
            <div className="p-6 space-y-4">
              {/* Provider Type */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Provider Type</label>
                <div className="flex items-center gap-3">
                  <div className="flex-1 px-3 py-2 bg-[#1e1e1e] border border-[#3c3c3c] rounded-md text-gray-200">
                    {currentProviderType}
                  </div>
                  <button
                    onClick={cycleProviderType}
                    className="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm rounded-md"
                  >
                    Cycle Type
                  </button>
                </div>
              </div>

              {/* Provider Name */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Name (optional)</label>
                <input
                  type="text"
                  className="w-full px-3 py-2 bg-[#1e1e1e] border border-[#3c3c3c] rounded-md text-gray-200 placeholder-gray-500 focus:outline-none focus:border-blue-500"
                  placeholder={`Custom name (default: ${currentProviderType})`}
                  value={providerName}
                  onChange={(e) => setProviderName(e.target.value)}
                />
              </div>

              {/* API Key */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">API Key</label>
                <input
                  type="password"
                  className="w-full px-3 py-2 bg-[#1e1e1e] border border-[#3c3c3c] rounded-md text-gray-200 placeholder-gray-500 focus:outline-none focus:border-blue-500"
                  placeholder="Enter your API key"
                  value={providerApiKey}
                  onChange={(e) => setProviderApiKey(e.target.value)}
                />
              </div>

              {/* Endpoint */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">API Endpoint</label>
                <input
                  type="text"
                  className="w-full px-3 py-2 bg-[#1e1e1e] border border-[#3c3c3c] rounded-md text-gray-200 placeholder-gray-500 focus:outline-none focus:border-blue-500"
                  placeholder={DEFAULT_ENDPOINTS[currentProviderType]}
                  value={providerEndpoint}
                  onChange={(e) => setProviderEndpoint(e.target.value)}
                />
              </div>

              {/* Model */}
              <div>
                <label className="block text-sm font-medium text-gray-300 mb-2">Model</label>
                <input
                  type="text"
                  className="w-full px-3 py-2 bg-[#1e1e1e] border border-[#3c3c3c] rounded-md text-gray-200 placeholder-gray-500 focus:outline-none focus:border-blue-500"
                  placeholder={DEFAULT_MODELS[currentProviderType]}
                  value={providerModel}
                  onChange={(e) => setProviderModel(e.target.value)}
                />
              </div>
            </div>

            {/* Footer */}
            <div className="flex items-center justify-end gap-3 p-4 border-t border-[#3c3c3c]">
              <button
                onClick={() => setShowProviderDialog(false)}
                className="px-4 py-2 bg-[#3c3c3c] hover:bg-[#4c4c4c] text-gray-200 text-sm rounded-md"
              >
                Cancel
              </button>
              <button
                onClick={addProvider}
                className="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white text-sm rounded-md"
              >
                Add Provider
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default App;

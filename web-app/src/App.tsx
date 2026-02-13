import { useState } from 'react'
import { Wallet, Zap, Send, RefreshCw, Copy, Github, Download, Terminal, Monitor, AlertCircle } from 'lucide-react'
import './App.css'

function App() {
  const [activeTab, setActiveTab] = useState<'demo' | 'download'>('demo')

  const demoAddress = 'tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx'

  const copyAddress = () => {
    navigator.clipboard.writeText(demoAddress)
    alert('Demo address copied to clipboard!')
  }

  return (
    <div className="min-h-screen p-6 bg-[var(--background)] text-[var(--text)]">
      {/* Alert Banner */}
      <div className="alert-warning mb-6 flex items-start gap-3">
        <AlertCircle className="w-5 h-5 flex-shrink-0 mt-0.5" />
        <div className="text-sm">
          <strong>Web Demo Version:</strong> This is a demonstration of the Unified Lightning Wallet UI.
          For full functionality including wallet creation, transactions, and Lightning Network features,
          please download the desktop application or use the CLI version.
        </div>
      </div>

      {/* Header */}
      <header className="mb-6 p-6 bg-gradient-to-r from-[var(--primary)] to-[var(--accent)] rounded-xl">
        <div className="flex justify-between items-center flex-wrap gap-4">
          <div>
            <h1 className="text-3xl font-bold text-white mb-2">⚡ Unified Lightning Wallet</h1>
            <p className="text-white/80 text-sm">Self-custodial Bitcoin & Lightning Network Wallet</p>
          </div>
          <div className="flex items-center gap-3 bg-white/20 px-4 py-2 rounded-lg">
            <Wallet className="w-5 h-5 text-white" />
            <span className="text-white font-bold">Web Demo</span>
          </div>
        </div>
      </header>

      {/* Navigation */}
      <div className="flex gap-3 mb-6">
        <button
          onClick={() => setActiveTab('demo')}
          className={`flex-1 flex items-center justify-center gap-2 p-4 rounded-lg transition ${
            activeTab === 'demo'
              ? 'bg-[var(--primary)] text-white'
              : 'bg-[var(--surface)] hover:bg-[var(--secondary)]'
          }`}
        >
          <Monitor className="w-5 h-5" />
          UI Demo
        </button>
        <button
          onClick={() => setActiveTab('download')}
          className={`flex-1 flex items-center justify-center gap-2 p-4 rounded-lg transition ${
            activeTab === 'download'
              ? 'bg-[var(--primary)] text-white'
              : 'bg-[var(--surface)] hover:bg-[var(--secondary)]'
          }`}
        >
          <Download className="w-5 h-5" />
          Get Full Version
        </button>
      </div>

      {activeTab === 'demo' ? (
        /* Demo Tab */
        <div className="space-y-4">
          {/* Balance Display */}
          <div className="bg-gradient-to-r from-purple-600 to-blue-600 p-6 rounded-lg text-white">
            <div className="text-sm opacity-80 mb-2">Demo Balance</div>
            <div className="text-4xl font-bold mb-2">0.00150000 BTC</div>
            <div className="text-sm opacity-70">≈ $67.50 USD (Demo)</div>
          </div>

          {/* Tabs */}
          <div className="bg-[var(--surface)] rounded-lg overflow-hidden">
            <div className="flex border-b border-[var(--secondary)]">
              <div className="flex-1 p-4 bg-[var(--primary)] text-white font-semibold text-center">
                <Wallet className="w-5 h-5 inline mr-2" />
                On-Chain
              </div>
              <div className="flex-1 p-4 text-center">
                <Zap className="w-5 h-5 inline mr-2" />
                Lightning
              </div>
            </div>

            <div className="p-6 space-y-6">
              {/* Receive Section */}
              <div>
                <h3 className="text-lg font-bold mb-3">Receive Bitcoin (Demo)</h3>
                <div className="flex gap-2 items-center">
                  <code className="flex-1 p-3 bg-[var(--background)] rounded text-sm overflow-x-auto">
                    {demoAddress}
                  </code>
                  <button
                    onClick={copyAddress}
                    className="p-3 bg-[var(--primary)] rounded hover:bg-[var(--accent)] transition"
                    title="Copy address"
                  >
                    <Copy className="w-5 h-5" />
                  </button>
                </div>
                <p className="text-xs text-gray-400 mt-2">
                  This is a testnet address for demonstration purposes only
                </p>
              </div>

              {/* Send Section */}
              <div>
                <h3 className="text-lg font-bold mb-3">Send Bitcoin (Demo UI)</h3>
                <input
                  type="text"
                  placeholder="Bitcoin Address"
                  disabled
                  className="w-full p-3 mb-3 bg-[var(--background)] border-2 border-[var(--secondary)] rounded text-white opacity-50"
                />
                <input
                  type="number"
                  placeholder="Amount (sats)"
                  disabled
                  className="w-full p-3 mb-3 bg-[var(--background)] border-2 border-[var(--secondary)] rounded text-white opacity-50"
                />
                <button
                  disabled
                  className="w-full flex items-center justify-center gap-2 p-3 bg-[var(--secondary)] rounded opacity-50 cursor-not-allowed"
                >
                  <Send className="w-5 h-5" />
                  Download Full Version to Send
                </button>
              </div>

              {/* Sync Section */}
              <div>
                <button
                  disabled
                  className="w-full flex items-center justify-center gap-2 p-3 bg-[var(--secondary)] rounded opacity-50 cursor-not-allowed"
                >
                  <RefreshCw className="w-5 h-5" />
                  Sync Wallet (Full Version Only)
                </button>
              </div>
            </div>
          </div>

          {/* Features */}
          <div className="bg-[var(--surface)] p-6 rounded-lg">
            <h3 className="text-lg font-bold mb-4">✨ Features</h3>
            <div className="grid md:grid-cols-2 gap-4">
              <div className="flex items-start gap-3">
                <div className="w-10 h-10 bg-green-500/20 rounded-lg flex items-center justify-center flex-shrink-0">
                  <Wallet className="w-5 h-5 text-green-500" />
                </div>
                <div>
                  <div className="font-semibold">On-Chain Wallet</div>
                  <div className="text-sm text-gray-400">BDK-powered Bitcoin wallet with HD key derivation</div>
                </div>
              </div>
              <div className="flex items-start gap-3">
                <div className="w-10 h-10 bg-yellow-500/20 rounded-lg flex items-center justify-center flex-shrink-0">
                  <Zap className="w-5 h-5 text-yellow-500" />
                </div>
                <div>
                  <div className="font-semibold">Lightning Network</div>
                  <div className="text-sm text-gray-400">LDK integration for instant payments (Desktop/CLI)</div>
                </div>
              </div>
              <div className="flex items-start gap-3">
                <div className="w-10 h-10 bg-blue-500/20 rounded-lg flex items-center justify-center flex-shrink-0">
                  <Terminal className="w-5 h-5 text-blue-500" />
                </div>
                <div>
                  <div className="font-semibold">CLI & GUI</div>
                  <div className="text-sm text-gray-400">Command-line and desktop app versions available</div>
                </div>
              </div>
              <div className="flex items-start gap-3">
                <div className="w-10 h-10 bg-purple-500/20 rounded-lg flex items-center justify-center flex-shrink-0">
                  <Monitor className="w-5 h-5 text-purple-500" />
                </div>
                <div>
                  <div className="font-semibold">Self-Custodial</div>
                  <div className="text-sm text-gray-400">Your keys, your Bitcoin. Full custody and control</div>
                </div>
              </div>
            </div>
          </div>
        </div>
      ) : (
        /* Download Tab */
        <div className="space-y-6">
          <div className="bg-[var(--surface)] p-6 rounded-lg">
            <h2 className="text-2xl font-bold mb-4">Download Full Version</h2>
            <p className="text-gray-400 mb-6">
              Get the complete Unified Lightning Wallet with full functionality including
              wallet creation, transaction signing, and Lightning Network support.
            </p>

            <div className="grid md:grid-cols-2 gap-4 mb-6">
              {/* Desktop GUI */}
              <div className="border-2 border-[var(--primary)] rounded-lg p-6">
                <div className="flex items-center gap-3 mb-4">
                  <div className="w-12 h-12 bg-[var(--primary)] rounded-lg flex items-center justify-center">
                    <Monitor className="w-6 h-6 text-white" />
                  </div>
                  <div>
                    <div className="font-bold text-lg">Desktop GUI</div>
                    <div className="text-sm text-gray-400">Tauri + React</div>
                  </div>
                </div>
                <p className="text-sm text-gray-400 mb-4">
                  Beautiful desktop application for Windows, macOS, and Linux
                </p>
                <a
                  href="https://github.com/chaitanya21kumar/unified-lightning-wallet/releases"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="block w-full text-center p-3 bg-[var(--primary)] rounded hover:bg-[var(--accent)] transition"
                >
                  Download Desktop App
                </a>
              </div>

              {/* CLI */}
              <div className="border-2 border-blue-500 rounded-lg p-6">
                <div className="flex items-center gap-3 mb-4">
                  <div className="w-12 h-12 bg-blue-500 rounded-lg flex items-center justify-center">
                    <Terminal className="w-6 h-6 text-white" />
                  </div>
                  <div>
                    <div className="font-bold text-lg">CLI Version</div>
                    <div className="text-sm text-gray-400">Command Line</div>
                  </div>
                </div>
                <p className="text-sm text-gray-400 mb-4">
                  Powerful command-line interface for advanced users
                </p>
                <a
                  href="https://github.com/chaitanya21kumar/unified-lightning-wallet#cli-usage"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="block w-full text-center p-3 bg-blue-500 rounded hover:bg-blue-600 transition"
                >
                  View CLI Docs
                </a>
              </div>
            </div>

            {/* Installation */}
            <div className="bg-[var(--background)] p-4 rounded-lg">
              <div className="font-bold mb-2">Quick Install (Rust/Cargo):</div>
              <code className="block p-3 bg-black/50 rounded text-sm overflow-x-auto">
                git clone https://github.com/chaitanya21kumar/unified-lightning-wallet.git<br />
                cd unified-lightning-wallet<br />
                cargo build --release
              </code>
            </div>
          </div>

          {/* GitHub */}
          <div className="bg-[var(--surface)] p-6 rounded-lg">
            <div className="flex items-center gap-3 mb-4">
              <Github className="w-8 h-8" />
              <div>
                <div className="font-bold text-lg">Open Source</div>
                <div className="text-sm text-gray-400">MIT Licensed • Built with Rust</div>
              </div>
            </div>
            <a
              href="https://github.com/chaitanya21kumar/unified-lightning-wallet"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center gap-2 px-6 py-3 bg-gray-700 hover:bg-gray-600 rounded transition"
            >
              <Github className="w-5 h-5" />
              View on GitHub
            </a>
          </div>

          {/* Tech Stack */}
          <div className="bg-[var(--surface)] p-6 rounded-lg">
            <h3 className="text-lg font-bold mb-4">Technology Stack</h3>
            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-sm">
              <div className="text-center p-3 bg-[var(--background)] rounded">
                <div className="font-bold text-orange-500">Rust</div>
                <div className="text-gray-400">Core Language</div>
              </div>
              <div className="text-center p-3 bg-[var(--background)] rounded">
                <div className="font-bold text-blue-500">BDK 1.0</div>
                <div className="text-gray-400">On-Chain</div>
              </div>
              <div className="text-center p-3 bg-[var(--background)] rounded">
                <div className="font-bold text-yellow-500">LDK</div>
                <div className="text-gray-400">Lightning</div>
              </div>
              <div className="text-center p-3 bg-[var(--background)] rounded">
                <div className="font-bold text-cyan-500">Tauri</div>
                <div className="text-gray-400">Desktop UI</div>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* Footer */}
      <footer className="mt-12 text-center text-sm text-gray-500">
        <p>Built for Summer of Bitcoin 2026 • MIT Licensed</p>
        <p className="mt-2">
          <a
            href="https://github.com/chaitanya21kumar/unified-lightning-wallet"
            target="_blank"
            rel="noopener noreferrer"
            className="text-[var(--primary)] hover:text-[var(--accent)]"
          >
            GitHub Repository
          </a>
          {' '} | {' '}
          <a
            href="https://github.com/chaitanya21kumar/unified-lightning-wallet/blob/main/README.md"
            target="_blank"
            rel="noopener noreferrer"
            className="text-[var(--primary)] hover:text-[var(--accent)]"
          >
            Documentation
          </a>
        </p>
      </footer>
    </div>
  )
}

export default App

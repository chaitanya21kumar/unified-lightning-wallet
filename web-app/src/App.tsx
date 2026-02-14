import { useState, useEffect } from 'react'
import { motion, AnimatePresence } from 'framer-motion'
import {
  Wallet, Zap, Send, Download, Terminal, Github,
  ArrowRight, Sparkles, Shield, Cpu,
  Globe, Eye, EyeOff, Copy, AlertCircle
} from 'lucide-react'
import { QRCodeSVG } from 'qrcode.react'
import { Toaster, toast } from 'react-hot-toast'
import './App.css'

function App() {
  const [showBalance, setShowBalance] = useState(true)
  const [activeView, setActiveView] = useState<'landing' | 'wallet'>('landing')
  const [walletTab, setWalletTab] = useState<'onchain' | 'lightning'>('onchain')
  const [sendAmount, setSendAmount] = useState('')
  const [receiveAddress] = useState('tb1qw508d6qejxtdg4y5r3zarvary0c5xw7kxpjzsx')

  // Particle effect
  const [particles, setParticles] = useState<Array<{ id: number; x: number; y: number }>>([])

  useEffect(() => {
    const newParticles = Array.from({ length: 20 }).map((_, i) => ({
      id: i,
      x: Math.random() * 100,
      y: Math.random() * 100,
    }))
    setParticles(newParticles)
  }, [])

  const copyToClipboard = (text: string, label: string) => {
    navigator.clipboard.writeText(text)
    toast.success(`${label} copied!`, {
      icon: '📋',
      style: {
        background: '#1e2440',
        color: '#fff',
        border: '1px solid #f7931a',
      },
    })
  }

  if (activeView === 'landing') {
    return (
      <div className="min-h-screen bg-dark-950 text-white overflow-hidden relative">
        <Toaster position="top-right" />

        {/* Animated background particles */}
        <div className="absolute inset-0 overflow-hidden pointer-events-none">
          {particles.map((particle) => (
            <motion.div
              key={particle.id}
              className="absolute w-2 h-2 bg-bitcoin-500/20 rounded-full"
              style={{
                left: `${particle.x}%`,
                top: `${particle.y}%`,
              }}
              animate={{
                y: [0, -30, 0],
                opacity: [0.2, 0.5, 0.2],
              }}
              transition={{
                duration: 3 + Math.random() * 2,
                repeat: Infinity,
                delay: Math.random() * 2,
              }}
            />
          ))}
        </div>

        {/* Gradient background */}
        <div className="absolute inset-0 bg-gradient-to-br from-dark-950 via-dark-900 to-dark-800 opacity-90" />

        {/* Main content */}
        <div className="relative z-10">
          {/* Header */}
          <motion.header
            initial={{ y: -100, opacity: 0 }}
            animate={{ y: 0, opacity: 1 }}
            className="container mx-auto px-6 py-6"
          >
            <nav className="flex justify-between items-center">
              <div className="flex items-center gap-3">
                <div className="w-12 h-12 bg-gradient-bitcoin rounded-xl flex items-center justify-center shadow-bitcoin">
                  <Zap className="w-7 h-7 text-white" />
                </div>
                <span className="text-2xl font-bold bg-gradient-bitcoin bg-clip-text text-transparent">
                  ULW
                </span>
              </div>
              <div className="flex items-center gap-4">
                <motion.a
                  href="https://github.com/chaitanya21kumar/unified-lightning-wallet"
                  target="_blank"
                  rel="noopener noreferrer"
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  className="px-6 py-2 bg-white/5 backdrop-blur-sm rounded-lg border border-white/10 hover:border-bitcoin-500/50 transition-all"
                >
                  <Github className="w-5 h-5 inline mr-2" />
                  GitHub
                </motion.a>
                <motion.button
                  onClick={() => setActiveView('wallet')}
                  whileHover={{ scale: 1.05 }}
                  whileTap={{ scale: 0.95 }}
                  className="px-6 py-2 bg-gradient-bitcoin rounded-lg font-semibold shadow-bitcoin hover:shadow-bitcoin-lg transition-all"
                >
                  Launch Demo
                </motion.button>
              </div>
            </nav>
          </motion.header>

          {/* Hero Section */}
          <motion.section
            initial={{ opacity: 0, y: 50 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ delay: 0.2 }}
            className="container mx-auto px-6 py-20 text-center"
          >
            <motion.div
              animate={{
                scale: [1, 1.02, 1],
              }}
              transition={{
                duration: 4,
                repeat: Infinity,
              }}
              className="inline-block mb-6"
            >
              <span className="px-4 py-2 bg-bitcoin-500/10 border border-bitcoin-500/30 rounded-full text-bitcoin-400 text-sm font-semibold">
                ⚡ Self-Custodial Bitcoin & Lightning Wallet
              </span>
            </motion.div>

            <h1 className="text-6xl md:text-8xl font-extrabold mb-8 leading-tight">
              <span className="bg-gradient-to-r from-white via-bitcoin-400 to-lightning-300 bg-clip-text text-transparent">
                Unified Lightning
              </span>
              <br />
              <span className="text-white">Wallet</span>
            </h1>

            <p className="text-xl md:text-2xl text-gray-300 mb-12 max-w-3xl mx-auto leading-relaxed">
              Experience the future of Bitcoin with our cutting-edge wallet combining
              <span className="text-bitcoin-500 font-semibold"> on-chain security</span> and
              <span className="text-lightning-400 font-semibold"> Lightning speed</span>.
            </p>

            <div className="flex flex-col sm:flex-row gap-4 justify-center items-center">
              <motion.button
                onClick={() => setActiveView('wallet')}
                whileHover={{ scale: 1.05, boxShadow: '0 0 30px rgba(247, 147, 26, 0.6)' }}
                whileTap={{ scale: 0.95 }}
                className="px-8 py-4 bg-gradient-bitcoin rounded-xl font-bold text-lg shadow-bitcoin flex items-center gap-3 group"
              >
                Try Demo Wallet
                <ArrowRight className="w-5 h-5 group-hover:translate-x-1 transition-transform" />
              </motion.button>

              <motion.a
                href="#download"
                whileHover={{ scale: 1.05 }}
                whileTap={{ scale: 0.95 }}
                className="px-8 py-4 bg-white/5 backdrop-blur-sm rounded-xl font-bold text-lg border border-white/10 hover:border-bitcoin-500/50 transition-all flex items-center gap-3"
              >
                <Download className="w-5 h-5" />
                Download App
              </motion.a>
            </div>
          </motion.section>

          {/* Features Section */}
          <motion.section
            initial={{ opacity: 0 }}
            whileInView={{ opacity: 1 }}
            viewport={{ once: true }}
            className="container mx-auto px-6 py-20"
          >
            <h2 className="text-4xl md:text-5xl font-bold text-center mb-16">
              <span className="bg-gradient-bitcoin bg-clip-text text-transparent">Why Choose ULW?</span>
            </h2>

            <div className="grid md:grid-cols-3 gap-8">
              {[
                {
                  icon: Shield,
                  title: 'Self-Custodial',
                  description: 'Your keys, your Bitcoin. Full control and sovereignty over your funds.',
                  gradient: 'from-blue-500 to-cyan-500',
                },
                {
                  icon: Zap,
                  title: 'Lightning Fast',
                  description: 'Instant payments with minimal fees using Lightning Network technology.',
                  gradient: 'from-bitcoin-500 to-lightning-400',
                },
                {
                  icon: Cpu,
                  title: 'Powered by Rust',
                  description: 'Built with BDK & LDK for maximum security and performance.',
                  gradient: 'from-purple-500 to-pink-500',
                },
              ].map((feature, index) => (
                <motion.div
                  key={index}
                  initial={{ opacity: 0, y: 50 }}
                  whileInView={{ opacity: 1, y: 0 }}
                  viewport={{ once: true }}
                  transition={{ delay: index * 0.1 }}
                  whileHover={{ y: -10, scale: 1.02 }}
                  className="relative group"
                >
                  <div className="absolute inset-0 bg-gradient-to-r opacity-0 group-hover:opacity-100 blur-xl transition-opacity duration-500 ${feature.gradient}" />
                  <div className="relative bg-white/5 backdrop-blur-xl rounded-2xl p-8 border border-white/10 group-hover:border-white/20 transition-all">
                    <div className={`inline-flex p-4 bg-gradient-to-r ${feature.gradient} rounded-xl mb-6 shadow-lg`}>
                      <feature.icon className="w-8 h-8 text-white" />
                    </div>
                    <h3 className="text-2xl font-bold mb-4">{feature.title}</h3>
                    <p className="text-gray-400 leading-relaxed">{feature.description}</p>
                  </div>
                </motion.div>
              ))}
            </div>
          </motion.section>

          {/* Download Section */}
          <motion.section
            id="download"
            initial={{ opacity: 0 }}
            whileInView={{ opacity: 1 }}
            viewport={{ once: true }}
            className="container mx-auto px-6 py-20"
          >
            <div className="bg-gradient-to-r from-dark-900 to-dark-800 rounded-3xl p-12 border border-bitcoin-500/20 relative overflow-hidden">
              <div className="absolute top-0 right-0 w-96 h-96 bg-bitcoin-500/10 rounded-full blur-3xl" />

              <div className="relative z-10">
                <h2 className="text-4xl md:text-5xl font-bold mb-6">
                  <span className="bg-gradient-bitcoin bg-clip-text text-transparent">Download Full Version</span>
                </h2>
                <p className="text-xl text-gray-300 mb-12 max-w-2xl">
                  Get the complete wallet with full functionality on desktop or command line.
                </p>

                <div className="grid md:grid-cols-2 gap-6">
                  <motion.div
                    whileHover={{ scale: 1.02 }}
                    className="bg-white/5 backdrop-blur-xl rounded-2xl p-8 border border-white/10"
                  >
                    <div className="flex items-center gap-4 mb-4">
                      <div className="p-3 bg-gradient-bitcoin rounded-xl">
                        <Download className="w-6 h-6 text-white" />
                      </div>
                      <div>
                        <h3 className="text-xl font-bold">Desktop GUI</h3>
                        <p className="text-sm text-gray-400">Windows • macOS • Linux</p>
                      </div>
                    </div>
                    <p className="text-gray-400 mb-6">Beautiful native application built with Tauri + React</p>
                    <a
                      href="https://github.com/chaitanya21kumar/unified-lightning-wallet/releases"
                      target="_blank"
                      rel="noopener noreferrer"
                      className="block text-center px-6 py-3 bg-gradient-bitcoin rounded-lg font-semibold hover:shadow-bitcoin transition-all"
                    >
                      Download Desktop App
                    </a>
                  </motion.div>

                  <motion.div
                    whileHover={{ scale: 1.02 }}
                    className="bg-white/5 backdrop-blur-xl rounded-2xl p-8 border border-white/10"
                  >
                    <div className="flex items-center gap-4 mb-4">
                      <div className="p-3 bg-gradient-to-r from-blue-500 to-cyan-500 rounded-xl">
                        <Terminal className="w-6 h-6 text-white" />
                      </div>
                      <div>
                        <h3 className="text-xl font-bold">CLI Version</h3>
                        <p className="text-sm text-gray-400">Command Line Interface</p>
                      </div>
                    </div>
                    <p className="text-gray-400 mb-6">Powerful CLI for developers and advanced users</p>
                    <a
                      href="https://github.com/chaitanya21kumar/unified-lightning-wallet#cli-usage"
                      target="_blank"
                      rel="noopener noreferrer"
                      className="block text-center px-6 py-3 bg-gradient-to-r from-blue-500 to-cyan-500 rounded-lg font-semibold hover:shadow-lg transition-all"
                    >
                      View CLI Docs
                    </a>
                  </motion.div>
                </div>
              </div>
            </div>
          </motion.section>

          {/* Footer */}
          <footer className="container mx-auto px-6 py-12 text-center text-gray-500">
            <p className="mb-4">Built for Summer of Bitcoin 2026 • Open Source • MIT Licensed</p>
            <div className="flex justify-center gap-6">
              <a href="https://github.com/chaitanya21kumar/unified-lightning-wallet" className="hover:text-bitcoin-500 transition-colors">
                GitHub
              </a>
              <a href="https://github.com/chaitanya21kumar/unified-lightning-wallet/blob/main/docs/USER_GUIDE.md" className="hover:text-bitcoin-500 transition-colors">
                Documentation
              </a>
              <a href="https://github.com/chaitanya21kumar/unified-lightning-wallet/blob/main/docs/ARCHITECTURE.md" className="hover:text-bitcoin-500 transition-colors">
                Architecture
              </a>
            </div>
          </footer>
        </div>
      </div>
    )
  }

  // Wallet Demo View
  return (
    <div className="min-h-screen bg-dark-950 text-white">
      <Toaster position="top-right" />

      {/* Warning Banner */}
      <div className="bg-bitcoin-500/10 border-y border-bitcoin-500/30 py-3">
        <div className="container mx-auto px-6 flex items-center justify-between">
          <div className="flex items-center gap-3">
            <AlertCircle className="w-5 h-5 text-bitcoin-400" />
            <p className="text-sm text-bitcoin-300">
              <strong>Demo Mode:</strong> This is a UI demonstration. Download the full app for actual wallet functionality.
            </p>
          </div>
          <motion.button
            onClick={() => setActiveView('landing')}
            whileHover={{ scale: 1.05 }}
            whileTap={{ scale: 0.95 }}
            className="text-sm px-4 py-2 bg-white/10 rounded-lg hover:bg-white/20 transition-colors"
          >
            Back to Home
          </motion.button>
        </div>
      </div>

      <div className="container mx-auto px-6 py-8">
        {/* Balance Card */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="bg-gradient-to-br from-bitcoin-600 to-lightning-600 rounded-3xl p-8 mb-8 shadow-bitcoin relative overflow-hidden"
        >
          <div className="absolute top-0 right-0 w-64 h-64 bg-white/10 rounded-full blur-3xl" />

          <div className="relative z-10">
            <div className="flex justify-between items-start mb-6">
              <div>
                <p className="text-white/80 text-sm mb-2">Total Balance</p>
                <div className="flex items-center gap-4">
                  {showBalance ? (
                    <h2 className="text-5xl font-bold text-white">0.00150000 BTC</h2>
                  ) : (
                    <h2 className="text-5xl font-bold text-white">••••••••</h2>
                  )}
                  <button
                    onClick={() => setShowBalance(!showBalance)}
                    className="p-2 bg-white/10 rounded-lg hover:bg-white/20 transition-colors"
                  >
                    {showBalance ? <EyeOff className="w-5 h-5" /> : <Eye className="w-5 h-5" />}
                  </button>
                </div>
                <p className="text-white/60 text-sm mt-2">≈ $67.50 USD (Demo)</p>
              </div>
              <div className="flex gap-2">
                <div className="px-3 py-1 bg-white/10 rounded-lg text-sm backdrop-blur-sm">
                  <Globe className="w-4 h-4 inline mr-1" />
                  Testnet
                </div>
              </div>
            </div>
          </div>
        </motion.div>

        {/* Tabs */}
        <div className="flex gap-4 mb-6">
          <motion.button
            onClick={() => setWalletTab('onchain')}
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
            className={`flex-1 py-4 rounded-xl font-semibold transition-all ${
              walletTab === 'onchain'
                ? 'bg-gradient-bitcoin shadow-bitcoin'
                : 'bg-white/5 hover:bg-white/10'
            }`}
          >
            <Wallet className="w-5 h-5 inline mr-2" />
            On-Chain
          </motion.button>
          <motion.button
            onClick={() => setWalletTab('lightning')}
            whileHover={{ scale: 1.02 }}
            whileTap={{ scale: 0.98 }}
            className={`flex-1 py-4 rounded-xl font-semibold transition-all ${
              walletTab === 'lightning'
                ? 'bg-gradient-lightning text-dark-950 shadow-lg'
                : 'bg-white/5 hover:bg-white/10'
            }`}
          >
            <Zap className="w-5 h-5 inline mr-2" />
            Lightning
          </motion.button>
        </div>

        {/* Wallet Content */}
        <AnimatePresence mode="wait">
          <motion.div
            key={walletTab}
            initial={{ opacity: 0, x: 20 }}
            animate={{ opacity: 1, x: 0 }}
            exit={{ opacity: 0, x: -20 }}
            className="grid md:grid-cols-2 gap-6"
          >
            {/* Receive Section */}
            <div className="bg-white/5 backdrop-blur-xl rounded-2xl p-6 border border-white/10">
              <h3 className="text-xl font-bold mb-4 flex items-center gap-2">
                <div className="p-2 bg-green-500/20 rounded-lg">
                  <Download className="w-5 h-5 text-green-400" />
                </div>
                Receive
              </h3>

              <div className="bg-white p-4 rounded-xl mb-4">
                <QRCodeSVG value={receiveAddress} size={200} className="mx-auto" />
              </div>

              <div className="flex gap-2">
                <input
                  type="text"
                  value={receiveAddress}
                  readOnly
                  className="flex-1 px-4 py-3 bg-dark-900 rounded-lg text-sm font-mono"
                />
                <button
                  onClick={() => copyToClipboard(receiveAddress, 'Address')}
                  className="p-3 bg-bitcoin-500 rounded-lg hover:bg-bitcoin-600 transition-colors"
                >
                  <Copy className="w-5 h-5" />
                </button>
              </div>
              <p className="text-xs text-gray-500 mt-2">Testnet address for demonstration</p>
            </div>

            {/* Send Section */}
            <div className="bg-white/5 backdrop-blur-xl rounded-2xl p-6 border border-white/10">
              <h3 className="text-xl font-bold mb-4 flex items-center gap-2">
                <div className="p-2 bg-bitcoin-500/20 rounded-lg">
                  <Send className="w-5 h-5 text-bitcoin-400" />
                </div>
                Send
              </h3>

              <div className="space-y-4">
                <div>
                  <label className="block text-sm text-gray-400 mb-2">Recipient Address</label>
                  <input
                    type="text"
                    placeholder="Enter Bitcoin address..."
                    disabled
                    className="w-full px-4 py-3 bg-dark-900/50 rounded-lg opacity-50 cursor-not-allowed"
                  />
                </div>

                <div>
                  <label className="block text-sm text-gray-400 mb-2">Amount</label>
                  <div className="relative">
                    <input
                      type="number"
                      placeholder="0.00000000"
                      value={sendAmount}
                      onChange={(e) => setSendAmount(e.target.value)}
                      disabled
                      className="w-full px-4 py-3 pr-16 bg-dark-900/50 rounded-lg opacity-50 cursor-not-allowed"
                    />
                    <span className="absolute right-4 top-1/2 -translate-y-1/2 text-gray-500 text-sm">BTC</span>
                  </div>
                </div>

                <button
                  disabled
                  className="w-full py-4 bg-gray-600 rounded-lg font-semibold opacity-50 cursor-not-allowed flex items-center justify-center gap-2"
                >
                  <Download className="w-5 h-5" />
                  Download Full App to Send
                </button>
              </div>
            </div>
          </motion.div>
        </AnimatePresence>

        {/* Transaction History */}
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.2 }}
          className="mt-8 bg-white/5 backdrop-blur-xl rounded-2xl p-6 border border-white/10"
        >
          <h3 className="text-xl font-bold mb-4">Recent Transactions</h3>
          <div className="text-center py-12 text-gray-500">
            <Sparkles className="w-12 h-12 mx-auto mb-4 opacity-50" />
            <p>No transactions yet</p>
            <p className="text-sm mt-2">Download the full app to start using your wallet</p>
          </div>
        </motion.div>
      </div>
    </div>
  )
}

export default App

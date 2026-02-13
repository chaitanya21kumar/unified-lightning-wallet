import { useState, useEffect } from 'react'
import { invoke } from '@tauri-apps/api/core'
import { Wallet, Zap, Send, RefreshCw, Copy } from 'lucide-react'
import './App.css'

interface WalletState {
  initialized: boolean
  balance: number
  address: string
  syncing: boolean
  transactions: string[]
}

function App() {
  const [wallet, setWallet] = useState<WalletState>({
    initialized: false,
    balance: 0,
    address: '',
    syncing: false,
    transactions: []
  })

  const [activeTab, setActiveTab] = useState<'onchain' | 'lightning'>('onchain')
  const [sendAddress, setSendAddress] = useState('')
  const [sendAmount, setSendAmount] = useState('')
  const [status, setStatus] = useState('')

  // Initialize wallet with demo descriptors on mount
  useEffect(() => {
    initDemoWallet()
  }, [])

  async function initDemoWallet() {
    try {
      setStatus('Initializing wallet...')
      // Demo descriptors for regtest (DO NOT use in production!)
      await invoke('init_wallet', {
        params: {
          network: 'regtest',
          descriptor: 'wpkh(tprv8ZgxMBicQKsPeZRHk4rTG6orPS2CRNFX3njhUXx5vj9qGog5ZMH4uGReDWN5kCkY3jmWEtWause41CDvBRXD1shKknAMKxT99o9qUTRVC6m/84\'/1\'/0\'/0/*)',
          changeDescriptor: 'wpkh(tprv8ZgxMBicQKsPeZRHk4rTG6orPS2CRNFX3njhUXx5vj9qGog5ZMH4uGReDWN5kCkY3jmWEtWause41CDvBRXD1shKknAMKxT99o9qUTRVC6m/84\'/1\'/0\'/1/*)',
          electrumUrl: 'tcp://localhost:50001'
        }
      })
      const balance = await invoke<number>('get_balance')
      const address = await invoke<string>('get_new_address')
      setWallet({ ...wallet, initialized: true, balance, address })
      setStatus('Wallet ready!')
    } catch (error) {
      setStatus(`Error: ${error}`)
    }
  }

  async function syncWallet() {
    setWallet(prev => ({ ...prev, syncing: true }))
    setStatus('Syncing...')
    try {
      await invoke('sync_wallet')
      const balance = await invoke<number>('get_balance')
      const txs = await invoke<string[]>('list_transactions')
      setWallet(prev => ({ ...prev, balance, transactions: txs, syncing: false }))
      setStatus('Sync complete!')
    } catch (error) {
      setStatus(`Sync error: ${error}`)
      setWallet(prev => ({ ...prev, syncing: false }))
    }
  }

  async function sendBitcoin() {
    if (!sendAddress || !sendAmount) {
      setStatus('Please enter address and amount')
      return
    }
    try {
      setStatus('Sending...')
      const txid = await invoke<string>('send_bitcoin', {
        address: sendAddress,
        amountSats: parseInt(sendAmount)
      })
      setStatus(`Sent! TXID: ${txid}`)
      setSendAddress('')
      setSendAmount('')
      await syncWallet()
    } catch (error) {
      setStatus(`Error: ${error}`)
    }
  }

  return (
    <div className="min-h-screen p-6 bg-[var(--background)] text-[var(--text)]">
      <header className="mb-6 p-6 bg-gradient-to-r from-[var(--primary)] to-[var(--accent)] rounded-xl">
        <div className="flex justify-between items-center">
          <h1 className="text-3xl font-bold text-white">⚡ Unified Lightning Wallet</h1>
          <div className="flex items-center gap-3 bg-white/20 px-4 py-2 rounded-lg">
            <Wallet className="w-5 h-5 text-white" />
            <span className="text-white font-bold">{(wallet.balance / 100000000).toFixed(8)} BTC</span>
          </div>
        </div>
      </header>

      <div className="mb-4 text-center text-sm font-semibold" style={{ color: wallet.initialized ? '#10b981' : '#f59e0b' }}>
        {status}
      </div>

      <div className="flex gap-3 mb-6">
        <button
          onClick={() => setActiveTab('onchain')}
          className={`flex-1 flex items-center justify-center gap-2 p-4 rounded-lg transition ${
            activeTab === 'onchain'
              ? 'bg-[var(--primary)] text-white'
              : 'bg-[var(--surface)] hover:bg-[var(--secondary)]'
          }`}
        >
          <Wallet className="w-5 h-5" />
          On-Chain
        </button>
        <button
          onClick={() => setActiveTab('lightning')}
          className={`flex-1 flex items-center justify-center gap-2 p-4 rounded-lg transition ${
            activeTab === 'lightning'
              ? 'bg-[var(--primary)] text-white'
              : 'bg-[var(--surface)] hover:bg-[var(--secondary)]'
          }`}
        >
          <Zap className="w-5 h-5" />
          Lightning
        </button>
      </div>

      {activeTab === 'onchain' ? (
        <div className="space-y-4">
          <div className="bg-[var(--surface)] p-6 rounded-lg">
            <h2 className="text-xl font-bold mb-4">Receive Bitcoin</h2>
            <div className="flex gap-2 items-center">
              <code className="flex-1 p-3 bg-[var(--background)] rounded text-sm overflow-x-auto">
                {wallet.address || 'Initializing...'}
              </code>
              <button
                onClick={() => navigator.clipboard.writeText(wallet.address)}
                className="p-3 bg-[var(--primary)] rounded hover:bg-[var(--accent)] transition"
              >
                <Copy className="w-5 h-5" />
              </button>
            </div>
          </div>

          <div className="bg-[var(--surface)] p-6 rounded-lg">
            <h2 className="text-xl font-bold mb-4">Send Bitcoin</h2>
            <input
              type="text"
              placeholder="Bitcoin Address"
              value={sendAddress}
              onChange={(e) => setSendAddress(e.target.value)}
              className="w-full p-3 mb-3 bg-[var(--background)] border-2 border-[var(--secondary)] rounded text-white"
            />
            <input
              type="number"
              placeholder="Amount (sats)"
              value={sendAmount}
              onChange={(e) => setSendAmount(e.target.value)}
              className="w-full p-3 mb-3 bg-[var(--background)] border-2 border-[var(--secondary)] rounded text-white"
            />
            <button
              onClick={sendBitcoin}
              className="w-full flex items-center justify-center gap-2 p-3 bg-[var(--primary)] rounded hover:bg-[var(--accent)] transition"
            >
              <Send className="w-5 h-5" />
              Send
            </button>
          </div>

          <div className="bg-[var(--surface)] p-6 rounded-lg">
            <button
              onClick={syncWallet}
              disabled={wallet.syncing}
              className="w-full flex items-center justify-center gap-2 p-3 bg-[var(--primary)] rounded hover:bg-[var(--accent)] transition disabled:opacity-50"
            >
              <RefreshCw className={`w-5 h-5 ${wallet.syncing ? 'spinning' : ''}`} />
              {wallet.syncing ? 'Syncing...' : 'Sync Wallet'}
            </button>
            {wallet.transactions.length > 0 && (
              <div className="mt-4">
                <h3 className="font-bold mb-2">Transactions:</h3>
                {wallet.transactions.map((tx, i) => (
                  <div key={i} className="text-xs p-2 bg-[var(--background)] rounded mb-1">
                    {tx}
                  </div>
                ))}
              </div>
            )}
          </div>
        </div>
      ) : (
        <div className="space-y-4">
          <div className="bg-[var(--surface)] p-6 rounded-lg text-center">
            <Zap className="w-16 h-16 mx-auto mb-4 text-[var(--accent)]" />
            <h2 className="text-xl font-bold mb-2">Lightning Network</h2>
            <p className="text-sm opacity-75">Coming soon in future Release!</p>
            <p className="text-xs mt-2 opacity-60">
              Open channels, create invoices, and make instant payments
            </p>
          </div>
        </div>
      )}
    </div>
  )
}

export default App

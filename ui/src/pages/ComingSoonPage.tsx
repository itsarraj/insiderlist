import { FormEvent, useState } from 'react'

const API_SUBSCRIBE = '/api/v1/subscribe'

export default function ComingSoonPage() {
  const [email, setEmail] = useState('')
  const [status, setStatus] = useState<'idle' | 'loading' | 'success' | 'error'>('idle')
  const [message, setMessage] = useState('')

  async function onSubmit(e: FormEvent) {
    e.preventDefault()
    setStatus('loading')
    setMessage('')
    try {
      const res = await fetch(API_SUBSCRIBE, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ email: email.trim() }),
      })
      const data = (await res.json()) as { ok?: boolean; message?: string }
      if (!res.ok) {
        setStatus('error')
        setMessage(data.message ?? 'Something went wrong.')
        return
      }
      setStatus('success')
      setMessage(data.message ?? "You're on the list.")
      setEmail('')
    } catch {
      setStatus('error')
      setMessage('Network error — check your connection and try again.')
    }
  }

  return (
    <div className="min-h-screen bg-zinc-950 text-zinc-100 flex flex-col">
      <div
        className="pointer-events-none fixed inset-0 opacity-[0.35]"
        style={{
          backgroundImage: `
            radial-gradient(ellipse 80% 50% at 50% -20%, rgba(120, 80, 220, 0.45), transparent),
            radial-gradient(ellipse 60% 40% at 100% 50%, rgba(59, 130, 246, 0.2), transparent)
          `,
        }}
      />
      <header className="relative z-10 px-6 py-8 md:px-12">
        <span className="text-sm font-medium tracking-[0.25em] uppercase text-zinc-500">
          InsiderList
        </span>
      </header>

      <main className="relative z-10 flex-1 flex items-center justify-center px-6 pb-24">
        <div className="w-full max-w-lg">
          <p className="text-amber-200/90 text-sm font-medium tracking-wide mb-4">Coming soon</p>
          <h1 className="text-4xl md:text-5xl font-semibold tracking-tight text-white leading-tight mb-6">
            Get on the insider list
          </h1>
          <p className="text-lg text-zinc-400 leading-relaxed mb-10">
            We are building something new. Drop your email and we will let you know when the doors
            open — no spam, one list, launch day only.
          </p>

          <form onSubmit={onSubmit} className="flex flex-col gap-4 sm:flex-row sm:items-stretch">
            <label htmlFor="email" className="sr-only">
              Email
            </label>
            <input
              id="email"
              name="email"
              type="email"
              autoComplete="email"
              required
              placeholder="you@example.com"
              value={email}
              onChange={(e) => setEmail(e.target.value)}
              disabled={status === 'loading'}
              className="flex-1 rounded-lg border border-zinc-700 bg-zinc-900/80 px-4 py-3 text-zinc-100 placeholder:text-zinc-600 outline-none ring-0 transition focus:border-violet-500/60 focus:ring-2 focus:ring-violet-500/30 disabled:opacity-60"
            />
            <button
              type="submit"
              disabled={status === 'loading'}
              className="rounded-lg bg-violet-600 px-6 py-3 font-medium text-white transition hover:bg-violet-500 disabled:opacity-60 disabled:hover:bg-violet-600"
            >
              {status === 'loading' ? 'Joining…' : 'Notify me'}
            </button>
          </form>

          {message && (
            <p
              className={`mt-4 text-sm ${status === 'error' ? 'text-red-400' : 'text-emerald-400'}`}
              role="status"
            >
              {message}
            </p>
          )}
        </div>
      </main>

      <footer className="relative z-10 px-6 py-8 text-center text-xs text-zinc-600 md:px-12">
        © {new Date().getFullYear()} InsiderList
      </footer>
    </div>
  )
}

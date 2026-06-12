export default function Home() {
  return (
    <main className="min-h-screen bg-gradient-to-br from-slate-50 to-slate-100">
      <nav className="border-b border-slate-200 bg-white/80 backdrop-blur-sm sticky top-0 z-50">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 flex justify-between items-center h-16">
          <h1 className="text-2xl font-bold">Irori 囲炉裏</h1>
          <a
            href="/api/docs"
            className="text-sm font-medium text-slate-600 hover:text-slate-900"
          >
            API Docs
          </a>
        </div>
      </nav>

      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-20">
        <div className="text-center mb-16">
          <h2 className="text-5xl font-bold text-slate-900 mb-4">
            A Shared Flame for Your Memories
          </h2>
          <p className="text-xl text-slate-600 max-w-2xl mx-auto mb-8">
            Family photo repository built in Rust. Self-hosted, private, and
            extensible. Like the traditional Japanese irori, Irori is the
            gathering place for your family's memories.
          </p>
          <div className="flex gap-4 justify-center">
            <a
              href="/api/docs"
              className="inline-flex items-center px-6 py-3 rounded-lg bg-slate-900 text-white hover:bg-slate-800 transition"
            >
              Explore API
            </a>
            <a
              href="https://github.com/saltyskip/irori"
              target="_blank"
              rel="noopener noreferrer"
              className="inline-flex items-center px-6 py-3 rounded-lg border border-slate-300 text-slate-900 hover:bg-slate-50 transition"
            >
              GitHub
            </a>
          </div>
        </div>

        <div className="grid grid-cols-1 md:grid-cols-3 gap-8 mb-16">
          <div className="bg-white rounded-lg p-6 border border-slate-200 hover:shadow-lg transition">
            <h3 className="text-lg font-semibold text-slate-900 mb-2">
              🔄 Protocol-Based Sync
            </h3>
            <p className="text-slate-600">
              Pluggable sync protocols (Irori native + Immich compatible) for
              multi-device synchronization.
            </p>
          </div>

          <div className="bg-white rounded-lg p-6 border border-slate-200 hover:shadow-lg transition">
            <h3 className="text-lg font-semibold text-slate-900 mb-2">
              👥 Family-First Sharing
            </h3>
            <p className="text-slate-600">
              Invite family members, assign roles (owner/editor/viewer), and
              manage permissions effortlessly.
            </p>
          </div>

          <div className="bg-white rounded-lg p-6 border border-slate-200 hover:shadow-lg transition">
            <h3 className="text-lg font-semibold text-slate-900 mb-2">
              📦 Flexible Storage
            </h3>
            <p className="text-slate-600">
              Store on your NAS, local filesystem, or S3-compatible services.
              Your data, your infrastructure.
            </p>
          </div>

          <div className="bg-white rounded-lg p-6 border border-slate-200 hover:shadow-lg transition">
            <h3 className="text-lg font-semibold text-slate-900 mb-2">
              🚀 Built for Performance
            </h3>
            <p className="text-slate-600">
              Written in Rust with Axum and PostgreSQL for speed and safety.
            </p>
          </div>

          <div className="bg-white rounded-lg p-6 border border-slate-200 hover:shadow-lg transition">
            <h3 className="text-lg font-semibold text-slate-900 mb-2">
              🔌 Extensible
            </h3>
            <p className="text-slate-600">
              Trait-based service layer allows swapping implementations and
              adding new features.
            </p>
          </div>

          <div className="bg-white rounded-lg p-6 border border-slate-200 hover:shadow-lg transition">
            <h3 className="text-lg font-semibold text-slate-900 mb-2">
              🏠 Self-Hosted
            </h3>
            <p className="text-slate-600">
              Run on your own infrastructure. Keep your family's memories
              private and under your control.
            </p>
          </div>
        </div>

        <div className="bg-white rounded-lg border border-slate-200 p-8">
          <h3 className="text-2xl font-semibold text-slate-900 mb-4">
            Why "Irori"?
          </h3>
          <p className="text-slate-600 leading-relaxed">
            囲炉裏 (irori) is a traditional Japanese sunken hearth, central to
            the Japanese home. Families gathered around it for warmth, cooking,
            and stories. It's the heart of the home—just like this project
            aspires to be the heart of your family's digital memories. Like the
            irori itself, Irori is designed to be a warm center where memories
            converge, with clear pathways for different approaches to reach it.
          </p>
        </div>
      </div>

      <footer className="border-t border-slate-200 bg-white py-8 mt-20">
        <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center text-slate-600 text-sm">
          <p>
            Irori — A shared hub for your memories. Built with Rust, Axum, and
            PostgreSQL.
          </p>
          <p className="mt-2">
            Dual licensed under MIT OR Apache-2.0 •{" "}
            <a
              href="https://github.com/saltyskip/irori"
              className="hover:text-slate-900"
            >
              GitHub
            </a>
          </p>
        </div>
      </footer>
    </main>
  );
}

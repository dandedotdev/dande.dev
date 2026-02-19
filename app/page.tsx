import Image from "next/image"

const SOCIAL_LINKS = [
  {
    label: "Instagram",
    href: "https://www.instagram.com/dandelion.huang/",
    icon: (
      <svg className="size-4" fill="currentColor" viewBox="0 0 24 24">
        <path d="M12 2.163c3.204 0 3.584.012 4.85.07 1.17.054 1.97.24 2.43.403a4.1 4.1 0 0 1 1.523.99 4.1 4.1 0 0 1 .99 1.524c.163.46.349 1.26.403 2.43.058 1.265.07 1.645.07 4.849s-.012 3.584-.07 4.849c-.054 1.17-.24 1.97-.403 2.43a4.1 4.1 0 0 1-.99 1.524 4.1 4.1 0 0 1-1.524.99c-.46.163-1.26.349-2.43.403-1.265.058-1.645.07-4.849.07s-3.584-.012-4.849-.07c-1.17-.054-1.97-.24-2.43-.403a4.1 4.1 0 0 1-1.524-.99 4.1 4.1 0 0 1-.99-1.524c-.163-.46-.349-1.26-.403-2.43C2.175 15.747 2.163 15.367 2.163 12.163s.012-3.584.07-4.849c.054-1.17.24-1.97.403-2.43a4.1 4.1 0 0 1 .99-1.524A4.1 4.1 0 0 1 5.15 2.37c.46-.163 1.26-.349 2.43-.403C8.845 1.909 9.225 1.897 12.43 1.897M12 0C8.741 0 8.333.014 7.053.072 5.775.13 4.903.333 4.14.63a5.876 5.876 0 0 0-2.126 1.384A5.876 5.876 0 0 0 .63 4.14C.333 4.903.13 5.775.072 7.053.014 8.333 0 8.741 0 12s.014 3.667.072 4.947c.058 1.278.261 2.15.558 2.913a5.876 5.876 0 0 0 1.384 2.126A5.876 5.876 0 0 0 4.14 23.37c.763.297 1.635.5 2.913.558C8.333 23.986 8.741 24 12 24s3.667-.014 4.947-.072c1.278-.058 2.15-.261 2.913-.558a5.876 5.876 0 0 0 2.126-1.384 5.876 5.876 0 0 0 1.384-2.126c.297-.763.5-1.635.558-2.913.058-1.28.072-1.688.072-4.947s-.014-3.667-.072-4.947c-.058-1.278-.261-2.15-.558-2.913a5.876 5.876 0 0 0-1.384-2.126A5.876 5.876 0 0 0 19.86.63c-.763-.297-1.635-.5-2.913-.558C15.667.014 15.259 0 12 0zm0 5.838a6.162 6.162 0 1 0 0 12.324 6.162 6.162 0 0 0 0-12.324zM12 16a4 4 0 1 1 0-8 4 4 0 0 1 0 8zm6.406-11.845a1.44 1.44 0 1 0 0 2.881 1.44 1.44 0 0 0 0-2.881z" />
      </svg>
    ),
  },
  {
    label: "GitHub",
    href: "https://github.com/dandedotdev",
    icon: (
      <svg className="size-4" fill="currentColor" viewBox="0 0 24 24">
        <path d="M12 2C6.477 2 2 6.484 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.158-1.11-1.466-1.11-1.466-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.338-2.22-.253-4.555-1.113-4.555-4.951 0-1.093.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0 1 12 6.844a9.59 9.59 0 0 1 2.504.337c1.909-1.296 2.747-1.027 2.747-1.027.546 1.379.202 2.398.1 2.651.64.7 1.028 1.595 1.028 2.688 0 3.848-2.339 4.695-4.566 4.943.359.309.678.92.678 1.855 0 1.338-.012 2.419-.012 2.747 0 .268.18.58.688.482A10.02 10.02 0 0 0 22 12.017C22 6.484 17.522 2 12 2z" />
      </svg>
    ),
  },
] as const

export default function Home() {
  return (
    <div className="bg-background mx-auto flex min-h-svh max-w-xl flex-col justify-center px-6 py-20">
      <main>
        <div className="flex items-center gap-4">
          <Image
            alt="Dandelion Huang"
            className="rounded-sm"
            height={64}
            priority
            src="/avatar.webp"
            width={64}
          />
        </div>

        <h1 className="mt-8 text-3xl font-bold tracking-tight sm:text-4xl">
          Dandelion Huang
        </h1>

        <p className="text-muted mt-5 text-base/relaxed">
          Full-stack Engineer focused on building scalable applications with
          seamless user experiences, using React and Rust, with a focus on AI
          and System Design. Active in the tech community&mdash;catch me at the
          next Clojure, dbt, or Elixir Taiwan Meetup!
        </p>

        <p className="text-muted mt-3 text-base/relaxed">
          Beyond code, I take great pleasure in tea, coffee, and desserts, and I
          love sharing these sensory experiences with those around me.
        </p>

        <nav className="mt-8 flex items-center gap-6">
          {SOCIAL_LINKS.map((link) => (
            <a
              key={link.label}
              className="text-muted hover:text-accent flex items-center gap-2 text-sm font-medium transition-colors"
              href={link.href}
              rel="noopener noreferrer"
              target="_blank"
            >
              {link.icon}
              {link.label}
            </a>
          ))}
        </nav>
      </main>
    </div>
  )
}

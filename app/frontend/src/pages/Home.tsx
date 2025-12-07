import {HomeProps} from "../types/inertia-props.ts";

export default function Home({ title, user, stats }: HomeProps) {
  return (
    <div style={{ fontFamily: 'system-ui, sans-serif', padding: '2rem', maxWidth: '600px', margin: '0 auto' }}>
      <h1>{title}</h1>

      <section style={{ marginTop: '2rem', padding: '1rem', background: '#f5f5f5', borderRadius: '8px' }}>
        <h2>User Info</h2>
        <p><strong>Name:</strong> {user.name}</p>
        <p><strong>Email:</strong> {user.email}</p>
      </section>

      <section style={{ marginTop: '1rem', padding: '1rem', background: '#e8f4e8', borderRadius: '8px' }}>
        <h2>Stats</h2>
        <p><strong>Visits:</strong> {stats.visits.toLocaleString()}</p>
        <p><strong>Likes:</strong> {stats.likes.toLocaleString()}</p>
      </section>

      <p style={{ marginTop: '2rem', color: '#666' }}>
        This page is rendered with Inertia.js + React, served by Kit (Rust)
      </p>
    </div>
  )
}

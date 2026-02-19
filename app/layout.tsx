import type { Metadata } from "next"
import { Geist, Geist_Mono } from "next/font/google"

import "@/styles/globals.css"

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
})

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
})

export const metadata: Metadata = {
  title: "Dandelion Huang",
  description:
    "Full-stack Engineer focused on building scalable applications with seamless user experiences, using React and Rust, with a focus on AI and System Design.",
}

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode
}>) {
  return (
    <html lang="en">
      <head>
        <link href="/favicon.ico" rel="icon" type="image/x-icon"></link>
      </head>
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        {children}
      </body>
    </html>
  )
}

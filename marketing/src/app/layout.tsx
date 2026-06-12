import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "Irori — A shared hub for your memories",
  description:
    "Family photo repository built in Rust. Self-hosted, private, extensible.",
  viewport: "width=device-width, initial-scale=1",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="en">
      <body>{children}</body>
    </html>
  );
}

import type { Config } from "tailwindcss";

export default {
  content: [
    "./pages/**/*.{js,ts,jsx,tsx,mdx}",
    "./components/**/*.{js,ts,jsx,tsx,mdx}",
    "./app/**/*.{js,ts,jsx,tsx,mdx}",
  ],
  theme: {
    extend: {
      colors: {
        background: "#1B151D",
        secondary: "#705976",
        primary: "#A9C7DD",
        accent: "#FAF0EA",
        accent2: "#DF5436",
      },
    },
  },
  plugins: [],
} satisfies Config;

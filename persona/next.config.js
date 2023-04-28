/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  webpack(config) {
    config.experiments = { 
      asyncWebAssembly: true,
      layers: true
    }
    return config
  },
}

module.exports = nextConfig

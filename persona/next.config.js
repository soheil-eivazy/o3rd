/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  webpack(config) {
    config.experiments = { 
      asyncWebAssembly: true,
      layers: true
    }
    // config.module.rules.push({
    //   test: /\.(glsl|vs|fs|vert|frag)/,
    //   type: "asset/source",
    // })
    config.module.rules.push({
      test: /\.(glsl|vs|fs|vert|frag)$/i,
      use: ['raw-loader', 'glslify-loader'],
    });
    // config.module.rules.push({
    //   test: /\.(glsl|vs|fs|vert|frag)$/i,
    //   exclude: /node_modules/,
    //   use: [
    //     {
    //       loader: 'raw-loader',
    //       options: {
    //         esModule: false,
    //       },
    //     },
    //   ]
    // });

    return config
  },
}

module.exports = nextConfig

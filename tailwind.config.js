module.exports = {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      backgroundImage: {
        'promo-one': "url('/static/sm.gif')"
      }
    },
    fontFamily: {
      sans: ['Roboto Mono', 'sans-serif']
    }
  },
  plugins: []
};

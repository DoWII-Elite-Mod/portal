module.exports = {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      backgroundImage: {
        'promo-one': "url('/images/sm.gif')"
      }
    },
    fontFamily: {
      sans: ['Roboto Mono', 'sans-serif']
    }
  },
  // TODO: refactor unexpected require
  // eslint-disable-next-line global-require
  plugins: [require('daisyui')]
};

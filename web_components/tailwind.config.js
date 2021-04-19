const colors = require('tailwindcss/colors');

if (process.env.NODE_ENV === 'production') {
  const production = true;
} else {
  const production = false;
}

module.exports = {
  purge: {
    mode: 'all',
    preserveHtmlElements: true,
    enabled: production,
    content: ['./src/**/*.html', './src/**/*.js', './src/**/*.tsx'],
    defaultExtractor: content => content.match(/[A-Za-z0-9-_:/]+/g) || [],
  },
  darkMode: false, // or 'media' or 'class'
  theme: {
    extend: {
      colors: {
        'light-blue': colors.lightBlue,
        'cyan': colors.cyan,
      },
    },
  },
  variants: {
    extend: {},
  },
  plugins: [],
  future: {
    purgeLayersByDefault: true,
    removeDeprecatedGapUtilities: true,
  },
};

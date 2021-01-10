import { Config } from '@stencil/core';
import tailwindcss from 'tailwindcss';
import { postcss } from '@stencil/postcss';
import autoprefixer from 'autoprefixer';
import purgecss from '@fullhuman/postcss-purgecss';

export const config: Config = {
  plugins: [
    postcss({
      plugins: [
        autoprefixer(),
        tailwindcss(),
        purgecss({
          preserveHtmlElements: true, 
          content: [
            './src/**/*.html',
            './src/**/*.js',
            './src/**/*.tsx'
          ]
        })
      ]
    })
  ],
  devServer: {
    reloadStrategy: 'pageReload'
  },
  namespace: 'waiting-room',
  outputTargets: [
    {
      type: 'www',
      serviceWorker: null, // disable service workers
    },
  ],
};

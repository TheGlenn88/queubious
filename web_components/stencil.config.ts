import { Config } from '@stencil/core';
import tailwindcss from 'tailwindcss';
import { postcss } from '@stencil/postcss';
import autoprefixer from 'autoprefixer';

export const config: Config = {
  plugins: [
    postcss({
      plugins: [
        autoprefixer(),
        tailwindcss(),
      ],
    }),
  ],
  devServer: {
    reloadStrategy: 'pageReload',
  },
  namespace: 'waiting-room',
  outputTargets: [
    {
      type: 'www',
      serviceWorker: null, // disable service workers
    },
  ],
};

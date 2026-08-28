import { defineConfig } from 'vite'

// ADR 0002 : les artefacts ne sont pas dupliques dans le depot. `publicDir`
// pointe la sortie du pipeline, que la construction recopie dans `dist/`.
// `base: './'` rend la sortie servable a n'importe quelle profondeur d'URL
// (racine de domaine ou sous-chemin de GitHub Pages) sans chemin ecrit en dur.
export default defineConfig({
  publicDir: '../public',
  base: './',
  build: { outDir: 'dist', emptyOutDir: true },
})

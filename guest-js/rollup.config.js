import typescript from '@rollup/plugin-typescript'

export default [
  {
    input: 'index.ts',
    output: [
      {
        file: 'dist/index.js',
        format: 'es',
        sourcemap: true
      },
      {
        file: 'dist/index.cjs',
        format: 'cjs',
        sourcemap: true
      }
    ],
    plugins: [
      typescript({
        tsconfig: './tsconfig.json'
      })
    ],
    external: ['@tauri-apps/api/core']
  }
]

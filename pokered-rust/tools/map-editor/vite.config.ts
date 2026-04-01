import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'
import fs from 'fs'

const gfxRoot = path.resolve(__dirname, '../../../gfx')

export default defineConfig({
  plugins: [
    vue(),
    tailwindcss(),
    {
      name: 'serve-gfx',
      configureServer(server) {
        server.middlewares.use('/gfx', (req, res, next) => {
          const filePath = path.join(gfxRoot, decodeURIComponent(req.url || ''))
          if (fs.existsSync(filePath) && fs.statSync(filePath).isFile()) {
            res.setHeader('Content-Type', 'image/png')
            fs.createReadStream(filePath).pipe(res)
          } else {
            next()
          }
        })
      },
    },
  ],
})

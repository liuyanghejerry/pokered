import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'
import fs from 'fs'
import type { IncomingMessage, ServerResponse } from 'http'

const gfxRoot = path.resolve(__dirname, '../../../gfx')
const mapsRoot = path.resolve(__dirname, '../../crates/pokered-data/maps')

const TILESET_BST_FILES: Record<string, string> = {
  Overworld: 'overworld.bst',
  RedsHouse1: 'reds_house.bst',
  Mart: 'pokecenter.bst',
  Forest: 'forest.bst',
  RedsHouse2: 'reds_house.bst',
  Dojo: 'gym.bst',
  Pokecenter: 'pokecenter.bst',
  Gym: 'gym.bst',
  House: 'house.bst',
  ForestGate: 'gate.bst',
  Museum: 'gate.bst',
  Underground: 'underground.bst',
  Gate: 'gate.bst',
  Ship: 'ship.bst',
  ShipPort: 'ship_port.bst',
  Cemetery: 'cemetery.bst',
  Interior: 'interior.bst',
  Cavern: 'cavern.bst',
  Lobby: 'lobby.bst',
  Mansion: 'mansion.bst',
  Lab: 'lab.bst',
  Club: 'club.bst',
  Facility: 'facility.bst',
  Plateau: 'plateau.bst',
}

const TILESET_PASSABLE_TILES: Record<string, number[]> = {
  Overworld: [0x00, 0x10, 0x1B, 0x20, 0x21, 0x23, 0x2C, 0x2D, 0x2E, 0x30, 0x31, 0x33, 0x39, 0x3C, 0x3E, 0x52, 0x54, 0x58, 0x5B],
  RedsHouse1: [0x01, 0x02, 0x03, 0x11, 0x12, 0x13, 0x14, 0x1C, 0x1A],
  Mart: [0x11, 0x1A, 0x1C, 0x3C, 0x5E],
  Forest: [0x1E, 0x20, 0x2E, 0x30, 0x34, 0x37, 0x39, 0x3A, 0x40, 0x51, 0x52, 0x5A, 0x5C, 0x5E, 0x5F],
  RedsHouse2: [0x01, 0x02, 0x03, 0x11, 0x12, 0x13, 0x14, 0x1C, 0x1A],
  Dojo: [0x11, 0x16, 0x19, 0x2B, 0x3C, 0x3D, 0x3F, 0x4A, 0x4C, 0x4D, 0x03],
  Pokecenter: [0x11, 0x1A, 0x1C, 0x3C, 0x5E],
  Gym: [0x11, 0x16, 0x19, 0x2B, 0x3C, 0x3D, 0x3F, 0x4A, 0x4C, 0x4D, 0x03],
  House: [0x01, 0x12, 0x14, 0x28, 0x32, 0x37, 0x44, 0x54, 0x5C],
  ForestGate: [0x01, 0x12, 0x14, 0x1A, 0x1C, 0x37, 0x38, 0x3B, 0x3C, 0x5E],
  Museum: [0x01, 0x12, 0x14, 0x1A, 0x1C, 0x37, 0x38, 0x3B, 0x3C, 0x5E],
  Underground: [0x0B, 0x0C, 0x13, 0x15, 0x18],
  Gate: [0x01, 0x12, 0x14, 0x1A, 0x1C, 0x37, 0x38, 0x3B, 0x3C, 0x5E],
  Ship: [0x04, 0x0D, 0x17, 0x1D, 0x1E, 0x23, 0x34, 0x37, 0x39, 0x4A],
  ShipPort: [0x0A, 0x1A, 0x32, 0x3B],
  Cemetery: [0x01, 0x10, 0x13, 0x1B, 0x22, 0x42, 0x52],
  Interior: [0x04, 0x0F, 0x15, 0x1F, 0x3B, 0x45, 0x47, 0x55, 0x56],
  Cavern: [0x05, 0x15, 0x18, 0x1A, 0x20, 0x21, 0x22, 0x2A, 0x2D, 0x30],
  Lobby: [0x14, 0x17, 0x1A, 0x1C, 0x20, 0x38, 0x45],
  Mansion: [0x01, 0x05, 0x11, 0x12, 0x14, 0x1A, 0x1C, 0x2C, 0x53],
  Lab: [0x0C, 0x26, 0x16, 0x1E, 0x34, 0x37],
  Club: [0x0F, 0x1A, 0x1F, 0x26, 0x28, 0x29, 0x2C, 0x2D, 0x2E, 0x2F, 0x41],
  Facility: [0x01, 0x10, 0x11, 0x13, 0x1B, 0x20, 0x21, 0x22, 0x30, 0x31, 0x32, 0x42, 0x43, 0x48, 0x52, 0x55, 0x58, 0x5E],
  Plateau: [0x1B, 0x23, 0x2C, 0x2D, 0x3B, 0x45],
}

const BLOCK_SIZE = 16

function parseBst(bstPath: string): Record<number, number[]> {
  const buf = fs.readFileSync(bstPath)
  const numBlocks = Math.floor(buf.length / BLOCK_SIZE)
  const blocks: Record<number, number[]> = {}
  for (let i = 0; i < numBlocks; i++) {
    const tiles: number[] = []
    for (let j = 0; j < BLOCK_SIZE; j++) {
      tiles.push(buf[i * BLOCK_SIZE + j])
    }
    blocks[i] = tiles
  }
  return blocks
}

function sendJson(res: ServerResponse, data: unknown, status = 200) {
  res.writeHead(status, { 'Content-Type': 'application/json' })
  res.end(JSON.stringify(data))
}

function sendError(res: ServerResponse, msg: string, status = 404) {
  res.writeHead(status, { 'Content-Type': 'application/json' })
  res.end(JSON.stringify({ error: msg }))
}

function readBody(req: IncomingMessage): Promise<string> {
  return new Promise((resolve, reject) => {
    const chunks: Buffer[] = []
    req.on('data', (c: Buffer) => chunks.push(c))
    req.on('end', () => resolve(Buffer.concat(chunks).toString('utf-8')))
    req.on('error', reject)
  })
}

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
    {
      name: 'map-data-api',
      configureServer(server) {
        server.middlewares.use('/api', async (req: IncomingMessage, res: ServerResponse, next) => {
          const url = decodeURIComponent(req.url || '')
          const method = req.method || 'GET'

          if (url === '/maps' && method === 'GET') {
            const dirs = fs.readdirSync(mapsRoot, { withFileTypes: true })
              .filter(d => d.isDirectory())
              .map(d => d.name)
              .sort()
            sendJson(res, dirs)
            return
          }

          const mapFileMatch = url.match(/^\/maps\/([^/]+)\/(.+)$/)
          if (mapFileMatch) {
            const mapName = mapFileMatch[1]
            const fileName = mapFileMatch[2]
            const mapDir = path.join(mapsRoot, mapName)

            if (!fs.existsSync(mapDir)) {
              sendError(res, `Map "${mapName}" not found`)
              return
            }

            if (method === 'GET') {
              if (fileName === 'map.json' || fileName === 'script_config.json') {
                const filePath = path.join(mapDir, fileName)
                if (!fs.existsSync(filePath)) {
                  sendError(res, `${fileName} not found for ${mapName}`)
                  return
                }
                const content = fs.readFileSync(filePath, 'utf-8')
                sendJson(res, JSON.parse(content))
                return
              }
              if (fileName === 'map.blk') {
                const filePath = path.join(mapDir, 'map.blk')
                if (!fs.existsSync(filePath)) {
                  sendJson(res, [])
                  return
                }
                const buf = fs.readFileSync(filePath)
                sendJson(res, Array.from(buf))
                return
              }
              if (fileName === 'script.js') {
                const filePath = path.join(mapDir, 'script.js')
                if (!fs.existsSync(filePath)) {
                  res.writeHead(200, { 'Content-Type': 'text/plain' })
                  res.end('')
                  return
                }
                const content = fs.readFileSync(filePath, 'utf-8')
                res.writeHead(200, { 'Content-Type': 'text/plain' })
                res.end(content)
                return
              }
            }

            if (method === 'PUT') {
              if (fileName === 'map.json' || fileName === 'script_config.json') {
                const body = await readBody(req)
                const parsed = JSON.parse(body)
                const filePath = path.join(mapDir, fileName)
                fs.writeFileSync(filePath, JSON.stringify(parsed, null, 2) + '\n')
                sendJson(res, { ok: true })
                return
              }
              if (fileName === 'map.blk') {
                const body = await readBody(req)
                const arr: number[] = JSON.parse(body)
                const buf = Buffer.from(arr)
                fs.writeFileSync(path.join(mapDir, 'map.blk'), buf)
                sendJson(res, { ok: true })
                return
              }
              if (fileName === 'script.js') {
                const body = await readBody(req)
                const filePath = path.join(mapDir, 'script.js')
                fs.writeFileSync(filePath, body)
                sendJson(res, { ok: true })
                return
              }
            }
          }

          if (url === '/blocksets' && method === 'GET') {
            const blocksets: Record<string, Record<number, number[]>> = {}
            for (const [name, file] of Object.entries(TILESET_BST_FILES)) {
              const bstPath = path.join(gfxRoot, 'blocksets', file)
              if (fs.existsSync(bstPath)) {
                blocksets[name] = parseBst(bstPath)
              }
            }
            sendJson(res, blocksets)
            return
          }

          if (url === '/passable-tiles' && method === 'GET') {
            sendJson(res, TILESET_PASSABLE_TILES)
            return
          }

          next()
        })
      },
    },
  ],
})

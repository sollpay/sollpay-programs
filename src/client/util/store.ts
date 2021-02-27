import * as path from 'path'
import * as fs from 'fs'

export class Store {
  dir = path.join(__dirname, '../../../store')

  async load(uri: string): Promise<Object> {
    const filename = path.join(this.dir, uri)
    const data = await fs.readFileSync(filename, 'utf8')
    const config = JSON.parse(data)
    return config
  }

  async save(uri: string, config: Object): Promise<void> {
    if (!fs.existsSync(this.dir)) {
      fs.mkdirSync(this.dir)
    }
    const filename = path.join(this.dir, uri)
    await fs.writeFileSync(filename, JSON.stringify(config), 'utf8')
  }
}

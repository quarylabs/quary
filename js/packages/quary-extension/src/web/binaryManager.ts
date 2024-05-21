import * as path from 'path'
import * as os from 'os'

export class BinaryManager {
  constructor(private extensionPath: string) {}

  getBinaryPath(): string {
    const platform = os.platform()
    const arch = os.arch()

    console.log({ platform, arch, extensionPath: this.extensionPath })

    let binarySubFolder = ''

    switch (platform) {
      case 'win32':
        binarySubFolder = 'win'
        break
      case 'darwin':
        binarySubFolder = arch === 'arm64' ? 'mac/arm' : 'mac/intel'
        break
      case 'linux':
        binarySubFolder = arch === 'arm64' ? 'linux/arm' : 'linux/x86_64'
        break
      default:
        throw new Error(`Unsupported platform: ${platform}`)
    }

    let binaryName = this.getBinaryName(platform)
    return path.join(this.extensionPath, 'bin', binarySubFolder, binaryName)
  }

  private getBinaryName(platform: NodeJS.Platform): string {
    return platform === 'win32' ? 'quary.exe' : 'quary'
  }
}

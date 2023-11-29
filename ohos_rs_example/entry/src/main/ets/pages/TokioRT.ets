import common from '@ohos.app.ability.common';
import fs from '@ohos.file.fs';
import tokioRT from 'libtokio_rt.so';
import buffer from '@ohos.buffer';
import log from '../utils/log';

let context = getContext(this) as common.UIAbilityContext;
let filesDir = context.filesDir;

@Entry
@Component
struct TokioRT {
  async writeAndReadFile() {
    // 新建并打开文件
    let file = fs.openSync(filesDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
    // 写入一段内容至文件
    fs.writeSync(file.fd, "Try to write str.");
    const result = await tokioRT.executeTokioReadFile(filesDir + '/test.txt');
    log.info(`Read file result: ${result.toString()}`)
  }

  async readFileFailed() {
    try {
      // 新建并打开文件
      let file = fs.openSync(filesDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
      // 写入一段内容至文件
      fs.writeSync(file.fd, "Try to write str.");
      await tokioRT.executeTokioReadFileFailed(filesDir + '/test.txt');
      throw new Error("Unreachable")
    } catch (e) {
      log.info(e.message);
    }
  }

  async readFileParalleled() {
    // 新建并打开文件
    let file = fs.openSync(filesDir + '/test.txt', fs.OpenMode.READ_WRITE | fs.OpenMode.CREATE);
    // 写入一段内容至文件
    fs.writeSync(file.fd, "Try to write str.");
    const buffers = await Promise.all(
      Array.from({ length: 50 }).map((_) => tokioRT.executeTokioReadFile(filesDir + '/test.txt'))
    );
    for (const fileContent of buffers) {
      log.info(`${fileContent.toString()}`)
    }
  }

  build() {
    Row() {
      Column({ space: 20 }) {
        Button('Read file').onClick(() => {
          this.writeAndReadFile();
        })
        Button("Read failed").onClick(() => {
          this.readFileFailed();
        })
        Button("Parallel").onClick(() => {
          this.readFileParalleled();
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}
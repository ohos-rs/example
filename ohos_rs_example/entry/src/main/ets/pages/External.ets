import external from 'libexternal.so';
import log from '../utils/log';

@Entry
@Component
struct External {
  build() {
    Row() {
      Column({ space: 20 }) {
        Button('Create External').onClick(() => {
          const obj = external.createExternal(42);
          log.info(`external ${external.getExternalCount(obj)}`)
        })
        Button('Create External With Size Hint').onClick(() => {
          const obj = external.createExternalWithHint(42);
          log.info(`external ${external.getExternalCount(obj)}`)
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}
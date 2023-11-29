import either from 'libeither.so';
import log from '../utils/log';

@Entry
@Component
struct Either {
  build() {
    Row() {
      Column({ space: 20 }) {
        Button('Either').onClick(() => {
          log.info(`either number: ${either.eitherNumberString(1)}`);
          log.info(`either string: ${either.eitherNumberString('napi')}`);
        })
        Button('Dynamic Arguments').onClick(() => {
          log.info(`dynamic arguments: ${either.dynamicArgumentLength(1)} ${either.dynamicArgumentLength()}`)
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}
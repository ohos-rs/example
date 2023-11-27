import basic from 'libbasic.so';
import log from '../utils/log';

@Entry
@Component
struct Basic {
  build() {
    Row() {
      Column({ space: 20 }) {
        Button('Get Boolean').onClick(() => {
          const b = basic.getBool();
          log.info(b.toString());
        })
        Button("Get Number").onClick(() => {
          const n = basic.getNumber();
          log.info(n.toString());
        })
        Button("Get Undefined").onClick(() => {
          const u = basic.getUndefined();
          log.info("undefined: ", String(typeof u === 'undefined'));
        })
        Button('Get Null').onClick(() => {
          const n = basic.getNull();
          log.info("null: ", String(typeof n === 'object' && n === null));
        })
        Button('Get Array').onClick(() => {
          const a = basic.getArray();
          log.info(String(a));
        })
        Button('Bigint').onClick(() => {
          const r = basic.bigintAdd(BigInt(100), BigInt(200));
          log.info(String(r));
        })
        Button("Get Object").onClick(() => {
          const o = basic.getObject();
          log.info(String(o.test));
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}
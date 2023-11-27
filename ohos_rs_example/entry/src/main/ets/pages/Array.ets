import nativeArray from 'libarray.so';
import log from '../utils/log';

@Entry
@Component
struct Array {
  build() {
    Row() {
      Column({ space: 20 }) {
        Button("createArray").onClick(() => {
          const a = nativeArray.createArray();
          log.info("createArray", String(a.length === 0 && typeof a === "object"));
        })
        Button("createArrayWithLength").onClick(() => {
          const a = nativeArray.createArrayWithLength(10);
          log.info("createArrayWithLength array", String(a.length === 10 && typeof a === "object"));
        })
        Button("setElement").onClick(() => {
          const a = nativeArray.setElement([1, 3, 4], 2, 2);
          log.info("setElement array", String(a.join("")));
        })
        Button("hasElement").onClick(() => {
          const a = nativeArray.hasElement([1, 2, 3, 4], 5);
          log.info("hasElement array", String(a));
        })
        Button("deleteElement").onClick(() => {
          const a = nativeArray.deleteElement([1, 2, 3, 4], 2);
          log.info("deleteElement array", String(a.join("")));
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}
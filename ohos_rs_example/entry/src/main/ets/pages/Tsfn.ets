import tsfn from 'libtsfn.so';

@Entry
@Component
struct Tsfn {
  @State message: string = '';

  build() {
    Row() {
      Column({ space: 20 }) {
        Text(this.message)
          .fontSize(50)
          .fontWeight(FontWeight.Bold)

        Button('Run TSFN').onClick(() => {
          tsfn.testTsFn((...args: number[]) => {
            this.message = `${args}`;
          })
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}
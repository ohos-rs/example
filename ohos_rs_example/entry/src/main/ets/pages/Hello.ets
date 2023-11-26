import hello from 'libhello.so';

@Entry
@Component
struct Hello {
  @State message: string = "";

  handleGetHello() {
    this.message = hello.hello();
  }

  build() {
    Row() {
      Column() {
        Text(this.message)
          .fontSize(50)
          .fontWeight(FontWeight.Bold)
        Button("Update hello").onClick(() => {
          this.handleGetHello();
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}
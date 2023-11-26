import router from '@ohos.router';

@Entry
@Component
struct Index {

  build() {
    Row() {
      Column() {
        List(){
          ListItem(){
            Button("Hello world").onClick(() => {
              router.pushUrl({
                url: 'pages/Hello'
              })
            })
          }
        }
      }
      .width('100%')
    }
    .height('100%')
  }
}

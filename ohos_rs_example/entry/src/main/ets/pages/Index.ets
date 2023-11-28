import router from '@ohos.router';

@Entry
@Component
struct Index {
  build() {
    Row() {
      Column({ space: 20 }) {
        Button("Hello world").onClick(() => {
          router.pushUrl({
            url: 'pages/Hello'
          })
        })
        Button("Basic Value").onClick(() => {
          router.pushUrl({
            url: 'pages/Basic'
          })
        })
        Button('Array').onClick(() => {
          router.pushUrl({
            url: 'pages/Array'
          })
        })
        Button("ArrayBuffer").onClick(() => {
          router.pushUrl({
            url: 'pages/ArrayBuffer'
          })
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}

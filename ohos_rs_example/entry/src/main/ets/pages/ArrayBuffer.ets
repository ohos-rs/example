import nativeArrayBuffer from 'libarraybuffer.so';
import buffer from '@ohos.buffer';
import log from '../utils/log';

@Entry
@Component
struct ArrayBuffer {
  build() {
    Row() {
      Column({ space: 20 }) {
        Button("getBufferLength").onClick(() => {
          const fixture = buffer.from('wow, hello');
          const length = nativeArrayBuffer.getArraybufferLength(fixture.buffer);
          log.info("getBufferLength",String(length), String(fixture.buffer.byteLength));
        })
        Button("mutateUint8Array").onClick(() => {
          const fixture = new Uint8Array([0, 1, 2]);
          nativeArrayBuffer.mutateUint8Array(fixture);
          log.info('mutateUint8Array', String(fixture[0] === 42));
        })
        Button("mutateUint16Array").onClick(() => {
          const fixture = new Uint16Array([0, 1, 2]);
          nativeArrayBuffer.mutateUint16Array(fixture);
          log.info('mutateUint16Array', String(fixture[0] === 65535));
        })
        Button("mutateInt16Array").onClick(() => {
          const fixture = new Int16Array([0, 1, 2]);
          nativeArrayBuffer.mutateInt16Array(fixture);
          log.info('mutateInt16Array', String(fixture[0] === 32767));
        })
        Button("mutateFloat32Array").onClick(() => {
          const fixture = new Float32Array([0, 1, 2]);
          nativeArrayBuffer.mutateFloat32Array(fixture);
          log.info('mutateFloat32Array', String(Math.abs(fixture[0] - 3.33) <= 0.0001));
        })
        Button("mutateFloat64Array").onClick(() => {
          const fixture = new Float64Array([0, 1, 2]);
          nativeArrayBuffer.mutateFloat64Array(fixture);
          log.info('mutateFloat64Array', String(Math.abs(fixture[0] - Math.PI) <= 0.0000001));
        })
        Button("mutateInt64Array").onClick(() => {
          const fixture = new BigInt64Array([BigInt(0), BigInt(1), BigInt(2)]);
          nativeArrayBuffer.mutateI64Array(fixture);
          log.info('mutateInt64Array', String(fixture[0]));
        })
      }
      .width('100%')
    }
    .height('100%')
  }
}
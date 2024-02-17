export class Matrix3 {
  constructor(private matrix: number[]) {}

  static identity(): Matrix3 {
    return new Matrix3([1, 0, 0, 0, 1, 0, 0, 0, 1]);
  }

  static projection(width: number, height: number): Matrix3 {
    return new Matrix3([2 / width, 0, 0, 0, -2 / height, 0, -1, 1, 1]);
  }

  static translation(x: number, y: number): Matrix3 {
    return new Matrix3([1, 0, 0, 0, 1, 0, x, y, 1]);
  }

  static scaling(scale: number): Matrix3 {
    return new Matrix3([scale, 0, 0, 0, scale, 0, 0, 0, 1]);
  }

  get x(): number {
    return this.matrix[6];
  }

  get y(): number {
    return this.matrix[7];
  }

  get zoom(): number {
    return this.matrix[0];
  }

  multiply(other: Matrix3): void {
    const [a00, a01, a02, a10, a11, a12, a20, a21, a22] = this.matrix;
    const [b00, b01, b02, b10, b11, b12, b20, b21, b22] = other.into();

    const dst = [
      b00 * a00 + b01 * a10 + b02 * a20,
      b00 * a01 + b01 * a11 + b02 * a21,
      b00 * a02 + b01 * a12 + b02 * a22,
      b10 * a00 + b11 * a10 + b12 * a20,
      b10 * a01 + b11 * a11 + b12 * a21,
      b10 * a02 + b11 * a12 + b12 * a22,
      b20 * a00 + b21 * a10 + b22 * a20,
      b20 * a01 + b21 * a11 + b22 * a21,
      b20 * a02 + b21 * a12 + b22 * a22,
    ];

    this.matrix = dst;
  }

  project(width: number, height: number): void {
    this.multiply(Matrix3.projection(width, height));
  }

  translate(x: number, y: number): void {
    this.multiply(Matrix3.translation(x, y));
  }

  scale(scale: number): void {
    this.multiply(Matrix3.scaling(scale));
  }

  inverse(): void {
    const m00 = this.matrix[0];
    const m01 = this.matrix[1];
    const m02 = this.matrix[2];
    const m10 = this.matrix[3];
    const m11 = this.matrix[4];
    const m12 = this.matrix[5];
    const m20 = this.matrix[6];
    const m21 = this.matrix[7];
    const m22 = this.matrix[8];

    const b01 = m22 * m11 - m12 * m21;
    const b11 = -m22 * m10 + m12 * m20;
    const b21 = m21 * m10 - m11 * m20;

    const det = m00 * b01 + m01 * b11 + m02 * b21;
    const invDet = 1.0 / det;

    const dst = [
      b01 * invDet,
      (-m22 * m01 + m02 * m21) * invDet,
      (m12 * m01 - m02 * m11) * invDet,
      b11 * invDet,
      (m22 * m00 - m02 * m20) * invDet,
      (-m12 * m00 + m02 * m10) * invDet,
      b21 * invDet,
      (-m21 * m00 + m01 * m20) * invDet,
      (m11 * m00 - m01 * m10) * invDet,
    ];

    this.matrix = dst;
  }

  transformPoint(x: number, y: number) {
    const d = x * this.matrix[2] + y * this.matrix[5] + this.matrix[8];
    return [
      (x * this.matrix[0] + y * this.matrix[3] + this.matrix[6]) / d,
      (x * this.matrix[1] + y * this.matrix[4] + this.matrix[7]) / d,
    ];
  }

  into(): Float32Array {
    return new Float32Array(this.matrix);
  }

  clone(): Matrix3 {
    return new Matrix3(this.matrix);
  }
}

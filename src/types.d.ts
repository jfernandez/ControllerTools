declare module "*.svg" {
  const content: string;
  export default content;
}

declare module "*.png" {
  const content: string;
  export default content;
}

declare module "*.jpg" {
  const content: string;
  export default content;
}

export interface IController {
  name: string;
  productId: number;
  vendorId: number;
  capacity: number;
  status: string;
  bluetooth: boolean;
}

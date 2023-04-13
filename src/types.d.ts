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


declare type MicroSDCard = {
  uid: string,
  name: string,
  games: string[]
}


declare type Game = {
  uid: string,
  name: string,
  size: number,
  card: string
}
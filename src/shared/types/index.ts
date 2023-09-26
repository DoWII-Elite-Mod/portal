export type Article = {
  title: string;
  content: string;
  date: string;
  tournament?: boolean;
};

export type TFactions =
  | 'Space Marines'
  | 'Chaos Space Marines'
  | 'Tyranids'
  | 'Orks'
  | 'Eldars'
  | 'Imperial Guard'
  | 'Ordo Malleus';

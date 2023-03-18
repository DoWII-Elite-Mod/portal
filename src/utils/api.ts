const baseUrl = process.env.BASE_URL ?? 'http://localhost:4000';

export async function fetchApi(path: string) {
  if (!path) {
    const message = 'No path given to fetchApi. Provide a relative path.';
    throw new Error(message);
  }

  return fetch(`${baseUrl}/api/${path}`);
}

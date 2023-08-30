import clsx, { ClassValue } from 'clsx';
import { twMerge } from 'tailwind-merge';

export const clsxm = (...classes: Array<ClassValue>) =>
  twMerge(clsx(...classes));

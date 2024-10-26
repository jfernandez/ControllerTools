export const log = (...args: any[]) => {
  console.log(
    `%c Decky %c ControllerTools %c`,
    'background: #16a085; color: black;',
    'background: #1abc9c; color: black;',
    'background: transparent;',
    ...args,
  );
};

export const error = (...args: any[]) => {
  console.error(
    `%c Decky %c ControllerTools %c`,
    'background: #16a085; color: black;',
    'background: #FF0000;',
    'background: transparent;',
    ...args,
  );
};
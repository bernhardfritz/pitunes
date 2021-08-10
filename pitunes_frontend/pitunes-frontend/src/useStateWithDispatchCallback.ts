import { SetStateAction, useEffect, useRef, useState } from 'react';

type SetStateCallback<T> = (state: T) => void;

type DispatchWithCallback<T> = (
  action: SetStateAction<T>,
  callback?: SetStateCallback<T>
) => void;

export const useStateWithDispatchCallback = <T>(
  initialState: T
): [T, DispatchWithCallback<T>] => {
  const callbackRef = useRef<SetStateCallback<T>>();
  const [state, setState] = useState(initialState);
  useEffect(() => {
    if (callbackRef.current) {
      callbackRef.current(state);
      callbackRef.current = undefined;
    }
  }, [state]);
  const setStateWithCallback: DispatchWithCallback<T> = (action, callback) => {
    callbackRef.current = callback;
    return setState(action);
  };
  return [state, setStateWithCallback];
};

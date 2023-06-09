import Lipe, { LoggerPipe, LogLevel } from "lipe"
import { PrefixWithColor, Splat, Timestamped } from "lipe/defaults"

const colors = {
    [LogLevel.Critical]: "#7d32d9",
    [LogLevel.Error]: "#d63a57",
    [LogLevel.Warn]: "#e3d462",
    [LogLevel.Log]: "#1c83e6",
    [LogLevel.Info]: "#95baed",
    [LogLevel.Debug]: "#b6c5d9",
}

const Pipe = new LoggerPipe()
    .Pipe(PrefixWithColor)
    .Pipe(Timestamped())
    .Pipe(Splat("%c MicroSDeck %c {prefix} %c {Message}"))
    .Pipe((message, {logLevel, args}) => {
        var logFunc = logLevel & (LogLevel.Critical | LogLevel.Error) ? console.error : console.log;

        logFunc(
            message,
            'background: #165da0; color: black;',
            `background: ${colors[logLevel]}; color: black;`,
            'background: transparent; color: white;',
            args
        );
    })


export const Logger = new Lipe().AddPipe(Pipe);
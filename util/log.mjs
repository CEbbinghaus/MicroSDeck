import Lipe, { LoggerPipe } from "lipe"
import { PrefixWithColor, Splat, Timestamped, Console } from "lipe/defaults"

const Pipe = new LoggerPipe()
	.Pipe(PrefixWithColor)
	.Pipe(Splat("{prefix} {Message}"))
	.Pipe(Console())


export const Logger = new Lipe().AddPipe(Pipe);
import { ServerAPI } from "decky-frontend-lib";

export class DeckyAPI {
	private static api: ServerAPI;

	public static SetApi(api: ServerAPI) {
		DeckyAPI.api = api;
	}

	public static Toast(title: string, message: string) {
		try {
			return DeckyAPI.api.toaster.toast({
				title: title,
				body: message,
				duration: 8000,
			});
		} catch (e) {
			console.log("Toaster Error", e);
		}
	}
}
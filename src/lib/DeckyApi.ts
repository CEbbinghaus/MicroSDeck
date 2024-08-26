import { toaster, ToastNotification } from "@decky/api"

export class DeckyAPI {
	public static Toast(title: string, message: string): ToastNotification | void {
		try {
			return toaster.toast({
				title: title,
				body: message,
				duration: 8000,
			});
		} catch (e) {
			console.log("Toaster Error", e);
		}
	}
}
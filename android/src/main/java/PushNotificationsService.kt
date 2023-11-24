package studio.darksoil.pushnotifications

import android.util.Log

class PushNotificationsService {
    fun pong(value: String): String {
        Log.i("Pong", value)
        return value
    }
}

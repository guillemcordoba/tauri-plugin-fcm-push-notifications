package studio.darksoil.pushnotifications

import android.util.Log
import com.google.firebase.messaging.FirebaseMessagingService
import com.google.firebase.messaging.RemoteMessage
import app.tauri.plugin.JSObject
import app.tauri.plugin.Channel

class PushNotificationsService(): FirebaseMessagingService()  {

    /**
     * Called if InstanceID token is updated. This may occur if the security of
     * the previous token had been compromised. Note that this is called when the InstanceID token
     * is initially generated so this is where you would retrieve the token.
     */
    override fun onNewToken(token: String) {
        super.onNewToken(token)
        Log.i("PushNotificationsService ", "Refreshed token :: $token")
        // If you want to send messages to this application instance or
        // manage this apps subscriptions on the server side, send the
        // Instance ID token to your app server.
        sendRegistrationToServer(token)
    }

    private fun sendRegistrationToServer(token: String) {
        // TODO : send token to tour server
    }


    companion object {
        init {
            System.loadLibrary("tauri_app_lib")
        }
    }

    override fun onMessageReceived(message: RemoteMessage) {
        super.onMessageReceived(message)
        Log.i("PushNotificationService ", "Message :: $message")

        val data = JSObject()

        for (entry in message.data.entries.iterator()) {
            data.put(entry.key, entry.value)
        }

        val message = JSObject()
        message.put("data", data)

        var tries = 0;

        if (PushNotificationsPlugin.channel == null){
            runn()

            while (PushNotificationsPlugin.channel == null && tries < 60) {
                Thread.sleep(500)  // wait for 1 second
                tries += 1

                Log.e("channel is ", "null")
            }
        }
        
        PushNotificationsPlugin.channel?.send(message)
    }

    private external fun runn()
}

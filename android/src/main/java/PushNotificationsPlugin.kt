package studio.darksoil.pushnotifications

import android.webkit.WebView
import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.Channel
import android.util.Log
import com.google.firebase.messaging.FirebaseMessaging
import com.google.firebase.FirebaseApp
import com.google.android.gms.tasks.OnCompleteListener

@InvokeArg
class SetupChannelArgs {
  lateinit var handler: Channel
}

@TauriPlugin
class PushNotificationsPlugin(private val activity: Activity): Plugin(activity) {

    @Command
    fun setupChannel(invoke: Invoke) {
        val args = invoke.parseArgs(SetupChannelArgs::class.java)
        PushNotificationsService.channel = args.handler
        invoke.resolve()
    }

    override fun onResume() {
        Log.e("ONRESUME", "" )
    }
  
    override fun load(webView: WebView) {
        FirebaseMessaging.getInstance().token.addOnCompleteListener(OnCompleteListener { task ->
            if (task.isSuccessful) {
                // Get new FCM registration token
                val token = task.result
                Log.e("myToken", "" + token)
            } else {  
                Log.w("Fetching FCM registration token failed", task.exception)
            }
        })
    }
}

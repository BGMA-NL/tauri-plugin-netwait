package nl.bgma.netwait

import android.app.Activity
import android.content.Context
import android.net.ConnectivityManager
import android.net.Network
import android.net.NetworkCapabilities
import android.net.NetworkRequest
import app.tauri.annotation.Command
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.ExperimentalCoroutinesApi
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import kotlinx.coroutines.suspendCancellableCoroutine
import kotlin.coroutines.resume

@TauriPlugin
class NetWaitPlugin(private val activity: Activity): Plugin(activity) {

    @OptIn(ExperimentalCoroutinesApi::class)
    suspend fun Context.waitForNetwork(): Boolean = suspendCancellableCoroutine { continuation ->
        val connectivityManager = getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager

        val network = connectivityManager.activeNetwork
        val capabilities = connectivityManager.getNetworkCapabilities(network)
        if (capabilities?.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET) == true) {
            continuation.resume(true)
            return@suspendCancellableCoroutine
        }

        val callback = object : ConnectivityManager.NetworkCallback() {
            override fun onAvailable(network: Network) {
                val capabilities = connectivityManager.getNetworkCapabilities(network)
                if (capabilities?.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET) == true) {
                    continuation.resume(true)
                    connectivityManager.unregisterNetworkCallback(this)
                }
            }
        }

        val request = NetworkRequest.Builder()
            .addCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET)
            .build()

        connectivityManager.registerNetworkCallback(request, callback)

        continuation.invokeOnCancellation {
            connectivityManager.unregisterNetworkCallback(callback)
        }
    }

    @Command
    fun checkNetworkStatus(invoke: Invoke) {
        val connectivityManager =
            activity.getSystemService(Context.CONNECTIVITY_SERVICE) as ConnectivityManager

        val network = connectivityManager.activeNetwork
        val capabilities = connectivityManager.getNetworkCapabilities(network)

        val isConnected = capabilities?.hasCapability(NetworkCapabilities.NET_CAPABILITY_INTERNET) == true
        val isValidated = capabilities?.hasCapability(NetworkCapabilities.NET_CAPABILITY_VALIDATED) == true

        val ret = JSObject()
        ret.put("is_connected", isConnected && isValidated)

        invoke.resolve(ret)
    }

    @OptIn(DelicateCoroutinesApi::class)
    @Command
    fun waitForNetwork(invoke: Invoke) {
        GlobalScope.launch {
            val isConnected = activity.waitForNetwork()

            val ret = JSObject()
            ret.put("is_connected", isConnected)

            invoke.resolve(ret)
        }
    }
}

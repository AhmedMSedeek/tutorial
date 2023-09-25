# reset and publish
resim reset
$command_output=resim new-account
$env:account=$command_output | Where-Object { $_ -match "Account component address:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$command_output=resim publish .
$env:package=$command_output | Where-Object { $_ -match "New Package:" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }

# create component
$command_output=resim run manifests\inistantiate_hello.rtm
$env:component=$command_output | Where-Object { $_ -match "component:" -And $_ -NotMatch "Address" } | ForEach-Object { $_.Split(":")[1].Replace(" ", "") }
$resources=$command_output | Where-Object { $_ -match "resource:" -And $_ -NotMatch "Address" }
$env:badge=$resources.Split(":")[1].Replace(" ", "")
$non_fungible_ids=$command_output | Where-Object { $_ -match "change:" -And $_ -match "{" }
$env:badge_id=$non_fungible_ids.Split(":")[1].Replace(" ", "").Replace("+{{", "{").Replace("}},-{}", "}")

# call x_function   // this succeeds
resim run manifests\x_function.rtm

# call y_function   // this one fails
resim run manifests\y_function.rtm

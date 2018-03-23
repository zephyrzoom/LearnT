from django.contrib.auth.models import User
from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.contrib.auth import authenticate, login, logout

import json

@csrf_exempt
def register(request):
    body = json.loads(request.body)
    req_username = body['username']
    alreadyExist = User.objects.filter(username=req_username)
    if not alreadyExist:
        req_password = body['password']
        user = User.objects.create_user(username=req_username, password=req_password)
        user.save()
        return JsonResponse({'info':'register success', 'username':req_username})
    else:
        return JsonResponse({'info':'register fail'})


@csrf_exempt
def mylogin(request):
    body = json.loads(request.body)
    req_username = body['username']
    req_password = body['password']
    user = authenticate(username=req_username, password=req_password)
    if user is not None:
        login(request, user)
        return JsonResponse({'info':'login'})
    else:
        return JsonResponse({'info':'log in fail'})

@csrf_exempt
def mylogout(request):
    logout(request)
    return JsonResponse({'info':'logout'})
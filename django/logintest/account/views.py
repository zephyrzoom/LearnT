from django.contrib.auth.models import User, Permission
from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.contrib.auth import authenticate, login, logout
from django.contrib.contenttypes.models import ContentType

from .models import App

import json

@csrf_exempt
def register(request):
    body = json.loads(request.body)
    req_username = body['username']
    alreadyExist = User.objects.filter(username=req_username)
    if not alreadyExist:
        req_password = body['password']
        user = User.objects.create(username=req_username, password=req_password)
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

@csrf_exempt
def get_permission(request):
    body = json.loads(request.body)
    req_username = body['username']
    req_password = body['password']
    user = authenticate(username=req_username, password=req_password)
    
    content_type = ContentType.objects.get_for_model(App)
    permission = Permission.objects.create(
        codename='can_register',
        name = 'Can Register',
        content_type = content_type,
    )
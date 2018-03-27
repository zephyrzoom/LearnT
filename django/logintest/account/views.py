from django.contrib.auth.models import User, Permission
from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.contrib.auth.decorators import login_required
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

@csrf_exempt
@login_required
def set_permission(request):
    body = json.loads(request.body)
    #req_username = body['username']
    #req_password = body['password']
    req_codename = body['codename']
    req_name     = body['name']
    user = request.user
    
    content_type = ContentType.objects.get_for_model(App)
    permission = Permission.objects.create(
        codename=req_codename,
        name = req_name,
        content_type = content_type,
    )
    user.user_permissions.add(permission)
    return JsonResponse({'info':'set permission'})

@login_required
@csrf_exempt
def get_permission(request):
    user = request.user
    return JsonResponse({'info':list(user.get_all_permissions())})

@csrf_exempt
def has_view_perm(request):
    if not request.user.is_authenticated:
        return JsonResponse({'info':'need login'})
    user=request.user
    body = json.loads(request.body)
    perm = body['perm']
    return JsonResponse({'info':user.has_perm(perm),'user':user.get_username()})
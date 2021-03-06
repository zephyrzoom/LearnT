from django.contrib.auth.models import Permission, Group
from django.http import JsonResponse
from django.views.decorators.csrf import csrf_exempt
from django.contrib.auth.decorators import login_required, permission_required
from django.contrib.auth import authenticate, login, logout, update_session_auth_hash
from django.contrib.contenttypes.models import ContentType
from django.db import transaction
from django.db.utils import Error
from django.contrib.sessions.models import Session

from .models import App, MyUser, MyBackend

import json

import logging
logger = logging.getLogger(__name__)

@csrf_exempt
@transaction.atomic
def register(request):
    body = json.loads(request.body)
    req_username = body['username']
    logger.info('---------------------username %s', req_username)
    logger.error('eeeeeeeeer')
    alreadyExist = MyUser.objects.filter(is_active=True).filter(username=req_username)

    if not alreadyExist:
        req_password = body['password']
        MyUser.objects.create_user(username=req_username, password=req_password)
        
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
    perms = user.get_all_permissions()
    return JsonResponse({'info':list(user.get_all_permissions())})

@csrf_exempt
def has_view_perm(request):
    if not request.user.is_authenticated:
        return JsonResponse({'info':'need login'})
    user=request.user
    body = json.loads(request.body)
    perm = body['perm']
    return JsonResponse({'info':user.has_perm(perm),'user':user.get_username()})

@permission_required('account.can view3')
@csrf_exempt
def test_perm(request):
    return JsonResponse({'info':'you have can view3 permission'})

@csrf_exempt
@login_required
def change_password(request):
    body = json.loads(request.body)
    old_pass = body['old_pass']
    new_pass = body['new_pass']
    user = request.user
    isUser = authenticate(username=user.username, password=old_pass)
    if isUser is not None:
        isUser.set_password(new_pass)
        isUser.save()
        update_session_auth_hash(request, isUser)
        return JsonResponse({'info':'password changed'})
    return JsonResponse({'info':'password change error'})


@csrf_exempt
def add_group(request):
    body = json.loads(request.body)
    group_name = body['group_name']
    isExist = Group.objects.filter(name=group_name)
    if isExist is not None:
        g = Group.objects.create(name=group_name)
        g.save()
        return JsonResponse({'info':'group name %s' % group_name})
    return JsonResponse({'info':'group already exist'})

@login_required
@csrf_exempt
def user_add_group(request):
    body = json.loads(request.body)
    group_name = body['group_name']
    db_group = Group.objects.get(name=group_name)
    user = request.user
    user.groups.add(db_group)
    return JsonResponse({'info':'added group'})

@csrf_exempt
def add_group_perm(request):
    body = json.loads(request.body)
    group_name = body['group_name']
    perm = body['perm']
    db_group = Group.objects.get(name=group_name)
    db_perm = Permission.objects.get(codename=perm)
    db_group.permissions.add(db_perm)
    return JsonResponse({'info':'added group permission'})

@csrf_exempt
def uploadapp(request):
    f = request.FILES['app']
    with open('test.txt', 'wb+') as app:
        for chunk in f.chunks():
            app.write(chunk)
    return JsonResponse({'info':'upload success'})

@csrf_exempt
def getSession(requust):
    s = Session.objects.all()
    ss = s[0].get_decoded()
    return JsonResponse({'info':ss})

@csrf_exempt
def getmethod(request):
    get = request.GET
    if get:
        try:
            from django.utils import timezone
            print(timezone.now())
            print(get['a'])
        except (KeyError):
            print('key error')
        
        return JsonResponse({'info':'ok'})
    else:
        return JsonResponse({'info':'bad'})
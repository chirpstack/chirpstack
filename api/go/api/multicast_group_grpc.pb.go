// Code generated by protoc-gen-go-grpc. DO NOT EDIT.
// versions:
// - protoc-gen-go-grpc v1.2.0
// - protoc             v3.18.1
// source: api/multicast_group.proto

package api

import (
	context "context"
	grpc "google.golang.org/grpc"
	codes "google.golang.org/grpc/codes"
	status "google.golang.org/grpc/status"
	emptypb "google.golang.org/protobuf/types/known/emptypb"
)

// This is a compile-time assertion to ensure that this generated file
// is compatible with the grpc package it is being compiled against.
// Requires gRPC-Go v1.32.0 or later.
const _ = grpc.SupportPackageIsVersion7

// MulticastGroupServiceClient is the client API for MulticastGroupService service.
//
// For semantics around ctx use and closing/ending streaming RPCs, please refer to https://pkg.go.dev/google.golang.org/grpc/?tab=doc#ClientConn.NewStream.
type MulticastGroupServiceClient interface {
	// Create the given multicast group.
	Create(ctx context.Context, in *CreateMulticastGroupRequest, opts ...grpc.CallOption) (*CreateMulticastGroupResponse, error)
	// Get returns the multicast group for the given ID.
	Get(ctx context.Context, in *GetMulticastGroupRequest, opts ...grpc.CallOption) (*GetMulticastGroupResponse, error)
	// Update the given multicast group.
	Update(ctx context.Context, in *UpdateMulticastGroupRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	// Delete the multicast-group with the given ID.
	Delete(ctx context.Context, in *DeleteMulticastGroupRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	// List the available multicast groups.
	List(ctx context.Context, in *ListMulticastGroupsRequest, opts ...grpc.CallOption) (*ListMulticastGroupsResponse, error)
	// Add a device to the multicast group.
	AddDevice(ctx context.Context, in *AddDeviceToMulticastGroupRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	// Remove a device from the multicast group.
	RemoveDevice(ctx context.Context, in *RemoveDeviceFromMulticastGroupRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	// Add the given item to the multcast group queue.
	Enqueue(ctx context.Context, in *EnqueueMulticastGroupQueueItemRequest, opts ...grpc.CallOption) (*EnqueueMulticastGroupQueueItemResponse, error)
	// Flush the queue for the given multicast group.
	FlushQueue(ctx context.Context, in *FlushMulticastGroupQueueRequest, opts ...grpc.CallOption) (*emptypb.Empty, error)
	// List the items in the multicast group queue.
	ListQueue(ctx context.Context, in *ListMulticastGroupQueueRequest, opts ...grpc.CallOption) (*ListMulticastGroupQueueResponse, error)
}

type multicastGroupServiceClient struct {
	cc grpc.ClientConnInterface
}

func NewMulticastGroupServiceClient(cc grpc.ClientConnInterface) MulticastGroupServiceClient {
	return &multicastGroupServiceClient{cc}
}

func (c *multicastGroupServiceClient) Create(ctx context.Context, in *CreateMulticastGroupRequest, opts ...grpc.CallOption) (*CreateMulticastGroupResponse, error) {
	out := new(CreateMulticastGroupResponse)
	err := c.cc.Invoke(ctx, "/api.MulticastGroupService/Create", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *multicastGroupServiceClient) Get(ctx context.Context, in *GetMulticastGroupRequest, opts ...grpc.CallOption) (*GetMulticastGroupResponse, error) {
	out := new(GetMulticastGroupResponse)
	err := c.cc.Invoke(ctx, "/api.MulticastGroupService/Get", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *multicastGroupServiceClient) Update(ctx context.Context, in *UpdateMulticastGroupRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, "/api.MulticastGroupService/Update", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *multicastGroupServiceClient) Delete(ctx context.Context, in *DeleteMulticastGroupRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, "/api.MulticastGroupService/Delete", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *multicastGroupServiceClient) List(ctx context.Context, in *ListMulticastGroupsRequest, opts ...grpc.CallOption) (*ListMulticastGroupsResponse, error) {
	out := new(ListMulticastGroupsResponse)
	err := c.cc.Invoke(ctx, "/api.MulticastGroupService/List", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *multicastGroupServiceClient) AddDevice(ctx context.Context, in *AddDeviceToMulticastGroupRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, "/api.MulticastGroupService/AddDevice", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *multicastGroupServiceClient) RemoveDevice(ctx context.Context, in *RemoveDeviceFromMulticastGroupRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, "/api.MulticastGroupService/RemoveDevice", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *multicastGroupServiceClient) Enqueue(ctx context.Context, in *EnqueueMulticastGroupQueueItemRequest, opts ...grpc.CallOption) (*EnqueueMulticastGroupQueueItemResponse, error) {
	out := new(EnqueueMulticastGroupQueueItemResponse)
	err := c.cc.Invoke(ctx, "/api.MulticastGroupService/Enqueue", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *multicastGroupServiceClient) FlushQueue(ctx context.Context, in *FlushMulticastGroupQueueRequest, opts ...grpc.CallOption) (*emptypb.Empty, error) {
	out := new(emptypb.Empty)
	err := c.cc.Invoke(ctx, "/api.MulticastGroupService/FlushQueue", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

func (c *multicastGroupServiceClient) ListQueue(ctx context.Context, in *ListMulticastGroupQueueRequest, opts ...grpc.CallOption) (*ListMulticastGroupQueueResponse, error) {
	out := new(ListMulticastGroupQueueResponse)
	err := c.cc.Invoke(ctx, "/api.MulticastGroupService/ListQueue", in, out, opts...)
	if err != nil {
		return nil, err
	}
	return out, nil
}

// MulticastGroupServiceServer is the server API for MulticastGroupService service.
// All implementations must embed UnimplementedMulticastGroupServiceServer
// for forward compatibility
type MulticastGroupServiceServer interface {
	// Create the given multicast group.
	Create(context.Context, *CreateMulticastGroupRequest) (*CreateMulticastGroupResponse, error)
	// Get returns the multicast group for the given ID.
	Get(context.Context, *GetMulticastGroupRequest) (*GetMulticastGroupResponse, error)
	// Update the given multicast group.
	Update(context.Context, *UpdateMulticastGroupRequest) (*emptypb.Empty, error)
	// Delete the multicast-group with the given ID.
	Delete(context.Context, *DeleteMulticastGroupRequest) (*emptypb.Empty, error)
	// List the available multicast groups.
	List(context.Context, *ListMulticastGroupsRequest) (*ListMulticastGroupsResponse, error)
	// Add a device to the multicast group.
	AddDevice(context.Context, *AddDeviceToMulticastGroupRequest) (*emptypb.Empty, error)
	// Remove a device from the multicast group.
	RemoveDevice(context.Context, *RemoveDeviceFromMulticastGroupRequest) (*emptypb.Empty, error)
	// Add the given item to the multcast group queue.
	Enqueue(context.Context, *EnqueueMulticastGroupQueueItemRequest) (*EnqueueMulticastGroupQueueItemResponse, error)
	// Flush the queue for the given multicast group.
	FlushQueue(context.Context, *FlushMulticastGroupQueueRequest) (*emptypb.Empty, error)
	// List the items in the multicast group queue.
	ListQueue(context.Context, *ListMulticastGroupQueueRequest) (*ListMulticastGroupQueueResponse, error)
	mustEmbedUnimplementedMulticastGroupServiceServer()
}

// UnimplementedMulticastGroupServiceServer must be embedded to have forward compatible implementations.
type UnimplementedMulticastGroupServiceServer struct {
}

func (UnimplementedMulticastGroupServiceServer) Create(context.Context, *CreateMulticastGroupRequest) (*CreateMulticastGroupResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Create not implemented")
}
func (UnimplementedMulticastGroupServiceServer) Get(context.Context, *GetMulticastGroupRequest) (*GetMulticastGroupResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Get not implemented")
}
func (UnimplementedMulticastGroupServiceServer) Update(context.Context, *UpdateMulticastGroupRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Update not implemented")
}
func (UnimplementedMulticastGroupServiceServer) Delete(context.Context, *DeleteMulticastGroupRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Delete not implemented")
}
func (UnimplementedMulticastGroupServiceServer) List(context.Context, *ListMulticastGroupsRequest) (*ListMulticastGroupsResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method List not implemented")
}
func (UnimplementedMulticastGroupServiceServer) AddDevice(context.Context, *AddDeviceToMulticastGroupRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method AddDevice not implemented")
}
func (UnimplementedMulticastGroupServiceServer) RemoveDevice(context.Context, *RemoveDeviceFromMulticastGroupRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method RemoveDevice not implemented")
}
func (UnimplementedMulticastGroupServiceServer) Enqueue(context.Context, *EnqueueMulticastGroupQueueItemRequest) (*EnqueueMulticastGroupQueueItemResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method Enqueue not implemented")
}
func (UnimplementedMulticastGroupServiceServer) FlushQueue(context.Context, *FlushMulticastGroupQueueRequest) (*emptypb.Empty, error) {
	return nil, status.Errorf(codes.Unimplemented, "method FlushQueue not implemented")
}
func (UnimplementedMulticastGroupServiceServer) ListQueue(context.Context, *ListMulticastGroupQueueRequest) (*ListMulticastGroupQueueResponse, error) {
	return nil, status.Errorf(codes.Unimplemented, "method ListQueue not implemented")
}
func (UnimplementedMulticastGroupServiceServer) mustEmbedUnimplementedMulticastGroupServiceServer() {}

// UnsafeMulticastGroupServiceServer may be embedded to opt out of forward compatibility for this service.
// Use of this interface is not recommended, as added methods to MulticastGroupServiceServer will
// result in compilation errors.
type UnsafeMulticastGroupServiceServer interface {
	mustEmbedUnimplementedMulticastGroupServiceServer()
}

func RegisterMulticastGroupServiceServer(s grpc.ServiceRegistrar, srv MulticastGroupServiceServer) {
	s.RegisterService(&MulticastGroupService_ServiceDesc, srv)
}

func _MulticastGroupService_Create_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(CreateMulticastGroupRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MulticastGroupServiceServer).Create(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.MulticastGroupService/Create",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MulticastGroupServiceServer).Create(ctx, req.(*CreateMulticastGroupRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _MulticastGroupService_Get_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(GetMulticastGroupRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MulticastGroupServiceServer).Get(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.MulticastGroupService/Get",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MulticastGroupServiceServer).Get(ctx, req.(*GetMulticastGroupRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _MulticastGroupService_Update_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(UpdateMulticastGroupRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MulticastGroupServiceServer).Update(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.MulticastGroupService/Update",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MulticastGroupServiceServer).Update(ctx, req.(*UpdateMulticastGroupRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _MulticastGroupService_Delete_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(DeleteMulticastGroupRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MulticastGroupServiceServer).Delete(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.MulticastGroupService/Delete",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MulticastGroupServiceServer).Delete(ctx, req.(*DeleteMulticastGroupRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _MulticastGroupService_List_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListMulticastGroupsRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MulticastGroupServiceServer).List(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.MulticastGroupService/List",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MulticastGroupServiceServer).List(ctx, req.(*ListMulticastGroupsRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _MulticastGroupService_AddDevice_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(AddDeviceToMulticastGroupRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MulticastGroupServiceServer).AddDevice(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.MulticastGroupService/AddDevice",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MulticastGroupServiceServer).AddDevice(ctx, req.(*AddDeviceToMulticastGroupRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _MulticastGroupService_RemoveDevice_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(RemoveDeviceFromMulticastGroupRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MulticastGroupServiceServer).RemoveDevice(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.MulticastGroupService/RemoveDevice",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MulticastGroupServiceServer).RemoveDevice(ctx, req.(*RemoveDeviceFromMulticastGroupRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _MulticastGroupService_Enqueue_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(EnqueueMulticastGroupQueueItemRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MulticastGroupServiceServer).Enqueue(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.MulticastGroupService/Enqueue",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MulticastGroupServiceServer).Enqueue(ctx, req.(*EnqueueMulticastGroupQueueItemRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _MulticastGroupService_FlushQueue_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(FlushMulticastGroupQueueRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MulticastGroupServiceServer).FlushQueue(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.MulticastGroupService/FlushQueue",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MulticastGroupServiceServer).FlushQueue(ctx, req.(*FlushMulticastGroupQueueRequest))
	}
	return interceptor(ctx, in, info, handler)
}

func _MulticastGroupService_ListQueue_Handler(srv interface{}, ctx context.Context, dec func(interface{}) error, interceptor grpc.UnaryServerInterceptor) (interface{}, error) {
	in := new(ListMulticastGroupQueueRequest)
	if err := dec(in); err != nil {
		return nil, err
	}
	if interceptor == nil {
		return srv.(MulticastGroupServiceServer).ListQueue(ctx, in)
	}
	info := &grpc.UnaryServerInfo{
		Server:     srv,
		FullMethod: "/api.MulticastGroupService/ListQueue",
	}
	handler := func(ctx context.Context, req interface{}) (interface{}, error) {
		return srv.(MulticastGroupServiceServer).ListQueue(ctx, req.(*ListMulticastGroupQueueRequest))
	}
	return interceptor(ctx, in, info, handler)
}

// MulticastGroupService_ServiceDesc is the grpc.ServiceDesc for MulticastGroupService service.
// It's only intended for direct use with grpc.RegisterService,
// and not to be introspected or modified (even as a copy)
var MulticastGroupService_ServiceDesc = grpc.ServiceDesc{
	ServiceName: "api.MulticastGroupService",
	HandlerType: (*MulticastGroupServiceServer)(nil),
	Methods: []grpc.MethodDesc{
		{
			MethodName: "Create",
			Handler:    _MulticastGroupService_Create_Handler,
		},
		{
			MethodName: "Get",
			Handler:    _MulticastGroupService_Get_Handler,
		},
		{
			MethodName: "Update",
			Handler:    _MulticastGroupService_Update_Handler,
		},
		{
			MethodName: "Delete",
			Handler:    _MulticastGroupService_Delete_Handler,
		},
		{
			MethodName: "List",
			Handler:    _MulticastGroupService_List_Handler,
		},
		{
			MethodName: "AddDevice",
			Handler:    _MulticastGroupService_AddDevice_Handler,
		},
		{
			MethodName: "RemoveDevice",
			Handler:    _MulticastGroupService_RemoveDevice_Handler,
		},
		{
			MethodName: "Enqueue",
			Handler:    _MulticastGroupService_Enqueue_Handler,
		},
		{
			MethodName: "FlushQueue",
			Handler:    _MulticastGroupService_FlushQueue_Handler,
		},
		{
			MethodName: "ListQueue",
			Handler:    _MulticastGroupService_ListQueue_Handler,
		},
	},
	Streams:  []grpc.StreamDesc{},
	Metadata: "api/multicast_group.proto",
}
